use std::fs::File;
use std::io::{BufReader, BufRead, stdin, stdout, Write};
use std::path::Path;

// ---------------------------
// 数据结构
// ---------------------------
#[derive(Debug)]
struct DataSet {
    features: Vec<Vec<f64>>,
    labels: Vec<f64>,
}

// ---------------------------
// CSV 读取和数据预处理
// ---------------------------
fn load_csv(path: &str) -> Result<DataSet, Box<dyn std::error::Error>> {
    println!("尝试加载文件: {}", path);
    
    let path = Path::new(path);
    if !path.exists() {
        // 尝试在当前目录下查找文件
        let file_name = path.file_name().unwrap_or_default().to_str().unwrap_or("titanic.csv");
        let current_dir = std::env::current_dir()?;
        let new_path = current_dir.join(file_name);
        
        println!("也在查找: {}", new_path.display());
        
        if new_path.exists() {
            println!("在当前位置找到文件: {}", new_path.display());
            return load_csv(new_path.to_str().unwrap());
        } else {
            // 列出当前目录下的文件帮助用户定位
            println!("当前目录下的文件:");
            if let Ok(entries) = std::fs::read_dir(&current_dir) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            println!("  - {}", entry.file_name().to_string_lossy());
                        }
                    }
                }
            }
            return Err(format!("文件不存在: {}. 请确保titanic.csv文件在正确位置", path.display()).into());
        }
    }
    
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut raw_rows = Vec::new();
    let mut ages = Vec::new();

    let mut line_count = 0;
    let mut skipped_lines = 0;

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        line_count += 1;
        
        if i == 0 {
            println!("CSV标题行: {}", line);
            continue; // skip header
        }

        let fields: Vec<String> = line.split(',').map(|s| s.trim_matches('"').trim().to_string()).collect();
        
        // 调试输出前几行
        if i <= 3 {
            println!("第{}行数据: {:?}", i, fields);
            println!("字段数量: {}", fields.len());
        }
        
        if fields.len() < 8 {
            println!("跳过不完整的行 {}: 只有 {} 个字段", i, fields.len());
            skipped_lines += 1;
            continue;
        }
        
        raw_rows.push(fields.clone());
        
        // 收集年龄用于计算中位数 - 根据你的CSV格式，年龄在第5个字段（索引4）
        if fields.len() > 4 && !fields[4].is_empty() && fields[4] != " " {
            if let Ok(age) = fields[4].parse::<f64>() {
                ages.push(age);
            }
        }
    }

    println!("总共读取 {} 行，跳过 {} 行不完整数据", line_count, skipped_lines);

    // 计算年龄中位数
    let median_age = if ages.is_empty() {
        println!("警告: 没有找到有效的年龄数据，使用默认值 29.7");
        29.7 // 默认值
    } else {
        ages.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = ages[ages.len() / 2];
        println!("计算得到年龄中位数: {}", median);
        median
    };

    let mut features = Vec::new();
    let mut labels = Vec::new();

    for (i, row) in raw_rows.iter().enumerate() {
        // 根据你的CSV格式调整字段索引：
        // ["survived", "pclass", "name", "sex", "age", "fare", "sibsp", "parch"]
        //    [0]        [1]      [2]     [3]    [4]    [5]     [6]      [7]
        
        // 解析标签 (survived) - 第1个字段
        let survived = if row[0].is_empty() { 
            0.0 
        } else { 
            row[0].parse::<f64>().unwrap_or(0.0) 
        };
        
        // 解析特征
        let pclass = row[1].parse::<f64>().unwrap_or(3.0);
        
        // 性别处理 - 第4个字段（索引3）
        let sex = if row[3].to_lowercase().contains("male") { 
            0.0 
        } else { 
            1.0 
        };
        
        // 年龄 - 第5个字段（索引4）
        let age = if row.len() > 4 && (row[4].is_empty() || row[4] == " ") {
            median_age
        } else if row.len() > 4 {
            row[4].parse::<f64>().unwrap_or(median_age)
        } else {
            median_age
        };
        
        // 票价 - 第6个字段（索引5）
        let fare = if row.len() > 5 && !row[5].is_empty() { 
            row[5].parse::<f64>().unwrap_or(0.0) 
        } else {
            0.0
        };
        
        // sibsp - 第7个字段（索引6）
        let sibsp = if row.len() > 6 { 
            row[6].parse::<f64>().unwrap_or(0.0) 
        } else {
            0.0
        };
        
        // parch - 第8个字段（索引7）
        let parch = if row.len() > 7 { 
            row[7].parse::<f64>().unwrap_or(0.0) 
        } else {
            0.0
        };
        
        let family_size = sibsp + parch + 1.0;

        features.push(vec![pclass, sex, age, fare, sibsp, parch, family_size]);
        labels.push(survived);
        
        // 显示前几条处理后的数据用于调试
        if i < 3 {
            println!("处理后的第{}条数据: pclass={}, sex={}, age={}, fare={}, sibsp={}, parch={}, survived={}", 
                     i, pclass, sex, age, fare, sibsp, parch, survived);
        }
    }

    println!("成功处理 {} 条有效数据", features.len());
    Ok(DataSet { features, labels })
}

// ---------------------------
// 训练/测试拆分
// ---------------------------
fn train_test_split(dataset: &DataSet, test_ratio: f64) -> (DataSet, DataSet) {
    let n = dataset.features.len();
    let test_size = (n as f64 * test_ratio) as usize;
    
    let mut x_train = Vec::new();
    let mut y_train = Vec::new();
    let mut x_test = Vec::new();
    let mut y_test = Vec::new();

    if n == 0 {
        return (DataSet { features: x_train, labels: y_train }, 
                DataSet { features: x_test, labels: y_test });
    }

    // 为了更好的随机性，我们可以打乱数据
    use std::collections::HashSet;
    let mut used_indices = HashSet::new();
    let mut test_indices = Vec::new();
    
    // 随机选择测试集索引
    while test_indices.len() < test_size {
        let idx = (test_indices.len() % n) as usize; // 简单轮询，实际应该用随机数
        if !used_indices.contains(&idx) {
            test_indices.push(idx);
            used_indices.insert(idx);
        }
    }
    
    for i in 0..n {
        if test_indices.contains(&i) {
            x_test.push(dataset.features[i].clone());
            y_test.push(dataset.labels[i]);
        } else {
            x_train.push(dataset.features[i].clone());
            y_train.push(dataset.labels[i]);
        }
    }

    (DataSet { features: x_train, labels: y_train }, 
     DataSet { features: x_test, labels: y_test })
}

// ---------------------------
// 逻辑回归
// ---------------------------
struct LogisticRegression {
    weights: Vec<f64>,
    lr: f64,
    epochs: usize,
}

impl LogisticRegression {
    fn new(n_features: usize, lr: f64, epochs: usize) -> Self {
        LogisticRegression {
            weights: vec![0.0; n_features + 1],
            lr,
            epochs,
        }
    }

    fn sigmoid(&self, z: f64) -> f64 {
        1.0 / (1.0 + (-z).exp())
    }

    fn fit(&mut self, x: &Vec<Vec<f64>>, y: &Vec<f64>) {
        let m = x.len();
        if m == 0 {
            println!("警告: 训练数据为空，跳过训练");
            return;
        }

        println!("开始训练逻辑回归，数据量: {}", m);
        
        for epoch in 0..self.epochs {
            let mut total_error = 0.0;
            
            for i in 0..m {
                let xi = &x[i];
                let mut z = self.weights[0]; // bias term
                
                for j in 0..xi.len() {
                    z += self.weights[j + 1] * xi[j];
                }
                
                let pred = self.sigmoid(z);
                let error = y[i] - pred;
                total_error += error.abs();

                // 更新权重
                self.weights[0] += self.lr * error;
                for j in 0..xi.len() {
                    self.weights[j + 1] += self.lr * error * xi[j];
                }
            }
            
            if epoch % 100 == 0 {
                println!("Epoch {}: 平均误差 = {:.4}", epoch, total_error / m as f64);
            }
        }
        
        println!("逻辑回归训练完成，最终权重: {:?}", self.weights);
    }

    fn predict(&self, x: &Vec<Vec<f64>>) -> Vec<f64> {
        if x.is_empty() {
            return Vec::new();
        }
        
        x.iter()
            .map(|xi| {
                let mut z = self.weights[0];
                for j in 0..xi.len() {
                    z += self.weights[j + 1] * xi[j];
                }
                let prob = self.sigmoid(z);
                if prob >= 0.5 {
                    1.0
                } else {
                    0.0
                }
            })
            .collect()
    }
    
    fn predict_proba(&self, x: &Vec<Vec<f64>>) -> Vec<f64> {
        if x.is_empty() {
            return Vec::new();
        }
        
        x.iter()
            .map(|xi| {
                let mut z = self.weights[0];
                for j in 0..xi.len() {
                    z += self.weights[j + 1] * xi[j];
                }
                self.sigmoid(z)
            })
            .collect()
    }
}

// ---------------------------
// 随机森林（极简版）
// ---------------------------
struct DecisionTree {
    feature_index: usize,
    threshold: f64,
    left_label: f64,
    right_label: f64,
}

impl DecisionTree {
    fn train(x: &Vec<Vec<f64>>, y: &Vec<f64>) -> Self {
        if x.is_empty() {
            return DecisionTree {
                feature_index: 0,
                threshold: 0.0,
                left_label: 0.0,
                right_label: 0.0,
            };
        }
        
        let n_features = x[0].len();
        let mut best_feature = 0;
        let mut best_thresh = 0.0;
        let mut best_score = -1.0;
        let mut left_label = 0.0;
        let mut right_label = 0.0;

        for f in 0..n_features {
            let mut vals: Vec<f64> = x.iter().map(|x_i| x_i[f]).collect();
            vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
            // 取一些候选阈值，避免处理所有唯一值
            let step = vals.len() / 10;
            if step == 0 { continue; }
            
            for i in (0..vals.len()).step_by(step) {
                let t = vals[i];
                let mut left_count = 0.0;
                let mut right_count = 0.0;
                let mut left_total = 0.0;
                let mut right_total = 0.0;

                for i in 0..x.len() {
                    if x[i][f] <= t {
                        left_total += 1.0;
                        left_count += y[i];
                    } else {
                        right_total += 1.0;
                        right_count += y[i];
                    }
                }

                // 避免除零
                if left_total == 0.0 || right_total == 0.0 {
                    continue;
                }
                
                let left_avg = left_count / left_total;
                let right_avg = right_count / right_total;
                
                // 使用信息增益作为评分
                let parent_impurity = 1.0 - (left_count + right_count).powi(2) / (left_total + right_total).powi(2);
                let left_impurity = 1.0 - left_avg.powi(2) - (1.0 - left_avg).powi(2);
                let right_impurity = 1.0 - right_avg.powi(2) - (1.0 - right_avg).powi(2);
                
                let info_gain = parent_impurity - (left_total * left_impurity + right_total * right_impurity) / (left_total + right_total);

                if info_gain > best_score {
                    best_score = info_gain;
                    best_feature = f;
                    best_thresh = t;
                    left_label = if left_avg >= 0.5 { 1.0 } else { 0.0 };
                    right_label = if right_avg >= 0.5 { 1.0 } else { 0.0 };
                }
            }
        }

        DecisionTree {
            feature_index: best_feature,
            threshold: best_thresh,
            left_label,
            right_label,
        }
    }

    fn predict(&self, x: &Vec<f64>) -> f64 {
        if x.is_empty() {
            return 0.0;
        }
        if x[self.feature_index] <= self.threshold {
            self.left_label
        } else {
            self.right_label
        }
    }
}

struct RandomForest {
    n_trees: usize,
    trees: Vec<DecisionTree>,
}

impl RandomForest {
    fn new(n_trees: usize) -> Self {
        RandomForest {
            n_trees,
            trees: Vec::new(),
        }
    }

    fn fit(&mut self, x: &Vec<Vec<f64>>, y: &Vec<f64>) {
        if x.is_empty() {
            println!("警告: 训练数据为空，跳过随机森林训练");
            return;
        }
        
        println!("开始训练随机森林，数据量: {}", x.len());
        
        for i in 0..self.n_trees {
            let tree = DecisionTree::train(x, y);
            self.trees.push(tree);
            if (i + 1) % 5 == 0 {
                println!("已训练 {} 棵树", i + 1);
            }
        }
    }

    fn predict(&self, x: &Vec<Vec<f64>>) -> Vec<f64> {
        if x.is_empty() {
            return Vec::new();
        }
        
        x.iter()
            .map(|xi| {
                let mut votes = 0.0;
                for tree in &self.trees {
                    votes += tree.predict(xi);
                }
                if votes / (self.trees.len() as f64) >= 0.5 {
                    1.0
                } else {
                    0.0
                }
            })
            .collect()
    }
    
    fn predict_proba(&self, x: &Vec<Vec<f64>>) -> Vec<f64> {
        if x.is_empty() {
            return Vec::new();
        }
        
        x.iter()
            .map(|xi| {
                let mut votes = 0.0;
                for tree in &self.trees {
                    votes += tree.predict(xi);
                }
                votes / (self.trees.len() as f64)
            })
            .collect()
    }
}

// ---------------------------
// 评估函数
// ---------------------------
fn accuracy_score(y_true: &Vec<f64>, y_pred: &Vec<f64>) -> f64 {
    if y_true.is_empty() || y_pred.is_empty() {
        return 0.0;
    }
    
    let mut correct = 0.0;
    for i in 0..y_true.len() {
        if (y_true[i] - y_pred[i]).abs() < 1e-6 {
            correct += 1.0;
        }
    }
    correct / y_true.len() as f64
}

// ---------------------------
// 交互预测
// ---------------------------
fn input_passenger() -> Option<Vec<f64>> {
    let mut input = String::new();
    
    println!("\n请输入新乘客信息预测生还概率:");
    
    print!("舱位等级(1/2/3): ");
    stdout().flush().unwrap();
    input.clear();
    stdin().read_line(&mut input).ok()?;
    let pclass: f64 = input.trim().parse().ok()?;
    if pclass < 1.0 || pclass > 3.0 {
        println!("舱位等级必须是1, 2或3");
        return None;
    }

    print!("性别(male/female): ");
    stdout().flush().unwrap();
    input.clear();
    stdin().read_line(&mut input).ok()?;
    let sex = if input.trim().to_lowercase() == "male" {
        0.0
    } else {
        1.0
    };

    print!("年龄: ");
    stdout().flush().unwrap();
    input.clear();
    stdin().read_line(&mut input).ok()?;
    let age: f64 = input.trim().parse().ok()?;

    print!("票价: ");
    stdout().flush().unwrap();
    input.clear();
    stdin().read_line(&mut input).ok()?;
    let fare: f64 = input.trim().parse().ok()?;

    print!("兄弟姐妹/配偶数: ");
    stdout().flush().unwrap();
    input.clear();
    stdin().read_line(&mut input).ok()?;
    let sibsp: f64 = input.trim().parse().ok()?;

    print!("父母/子女数: ");
    stdout().flush().unwrap();
    input.clear();
    stdin().read_line(&mut input).ok()?;
    let parch: f64 = input.trim().parse().ok()?;

    let family_size = sibsp + parch + 1.0;

    Some(vec![pclass, sex, age, fare, sibsp, parch, family_size])
}

// ---------------------------
// 主函数
// ---------------------------
fn main() {
    println!("泰坦尼克号生还预测系统");
    println!("=====================");
    
    // 加载数据 - 使用相对路径
    let dataset = match load_csv("titanic.csv") {
        Ok(data) => data,
        Err(e) => {
            println!("错误: {}", e);
            println!("\n请下载泰坦尼克号数据集并保存为 'titanic.csv'");
            println!("数据集下载地址: https://www.kaggle.com/c/titanic/data");
            println!("或者使用以下命令创建示例数据文件:");
            println!("echo 'survived,pclass,name,sex,age,fare,sibsp,parch' > titanic.csv");
            println!("echo '0,3,\"Braund, Mr. Owen Harris\",male,22,7.25,1,0' >> titanic.csv");
            println!("echo '1,1,\"Cumings, Mrs. John Bradley (Florence Briggs Thayer)\",female,38,71.2833,1,0' >> titanic.csv");
            return;
        }
    };
    
    if dataset.features.is_empty() {
        println!("错误: 没有加载到任何数据");
        println!("请检查CSV文件格式是否正确");
        return;
    }
    
    println!("成功加载 {} 条数据", dataset.features.len());
    
    // 拆分训练测试集
    let (train, test) = train_test_split(&dataset, 0.2);
    println!("训练集: {} 条, 测试集: {} 条", train.features.len(), test.features.len());

    if train.features.is_empty() {
        println!("错误: 训练集为空，无法训练模型");
        return;
    }

    // 训练逻辑回归
    println!("\n训练逻辑回归模型中...");
    let mut lr = LogisticRegression::new(train.features[0].len(), 0.01, 1000);
    lr.fit(&train.features, &train.labels);
    let lr_pred = lr.predict(&test.features);
    println!("逻辑回归准确率: {:.2}%", accuracy_score(&test.labels, &lr_pred) * 100.0);

    // 训练随机森林
    println!("\n训练随机森林模型中...");
    let mut rf = RandomForest::new(10); // 10棵树
    rf.fit(&train.features, &train.labels);
    let rf_pred = rf.predict(&test.features);
    println!("随机森林准确率: {:.2}%", accuracy_score(&test.labels, &rf_pred) * 100.0);

    // 交互预测
    println!("\n开始交互式预测...");
    loop {
        if let Some(x_new) = input_passenger() {
            let rf_proba = rf.predict_proba(&vec![x_new.clone()])[0];
            let rf_label = rf.predict(&vec![x_new.clone()])[0];
            let lr_proba = lr.predict_proba(&vec![x_new.clone()])[0];
            let lr_label = lr.predict(&vec![x_new.clone()])[0];
            
            println!("随机森林预测 - 生还概率: {:.2}%, 预测结果: {}", 
                     rf_proba * 100.0, if rf_label == 1.0 { "生还" } else { "未生还" });
            println!("逻辑回归预测 - 生还概率: {:.2}%, 预测结果: {}", 
                     lr_proba * 100.0, if lr_label == 1.0 { "生还" } else { "未生还" });
        } else {
            println!("输入有误，请重新输入");
            continue;
        }

        let mut input = String::new();
        print!("是否继续预测新乘客? (y/n): ");
        stdout().flush().unwrap();
        input.clear();
        stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            break;
        }
    }
    
    println!("感谢使用泰坦尼克号生还预测系统!");
}