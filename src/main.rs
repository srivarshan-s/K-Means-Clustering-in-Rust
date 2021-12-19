use rand::Rng;
// use std::io::*;

fn main() {
    // let (num_clusters, num_feat, num_points, feat) = init();
    let (num_clusters, num_points, feat) = hardcode_init();
    let mut cluster_points = init_cluster_points(&num_clusters, &num_points, &feat);
    let iter_num = 1000;
    cluster_points = k_means(cluster_points, &feat, iter_num);
    dbg!(&cluster_points);
}

fn k_means(cluster_points: Vec<Vec<f32>>, feat: &Vec<Vec<f32>>, iter_num: i32) -> Vec<Vec<f32>> {
    if iter_num == 0 {
        return cluster_points;
    }
    let mut cluster_new = Vec::<Vec<Vec<f32>>>::new();
    for _i in 0..cluster_points.len() {
        cluster_new.push(vec![vec![0.0]]);
    }
    for i in 0..feat.len() {
        let mut min_dist: f32 = get_dist(&cluster_points[0], &feat[i]);
        let mut min_dist_idx = 0;
        for j in 0..cluster_points.len() {
            if get_dist(&cluster_points[j], &feat[i]) < min_dist {
                min_dist = get_dist(&cluster_points[j], &feat[i]);
                min_dist_idx = j;
            }
        }
        cluster_new[min_dist_idx].push(feat[i].clone());
    }
    for i in 0..cluster_new.len() {
        cluster_new[i].remove(0);
    }
    let mut cluster_points_new = Vec::<Vec<f32>>::new();
    for _i in 0..cluster_points.len() {
        cluster_points_new.push(vec![0.0, 0.0]);
    }
    for i in 0..cluster_points_new.len() {
        for j in 0..cluster_points_new[i].len() {
            let mut sum: f32 = 0 as f32;
            for k in 0..cluster_new[i].len() {
                sum += cluster_new[i][k][j];
            }
            sum = sum / (cluster_new[i].len() as f32);
            cluster_points_new[i][j] = sum;
        }
    }
    k_means(cluster_points_new, feat, iter_num-1)
}

fn get_dist(point1: &Vec<f32>, point2: &Vec<f32>) -> f32 {
    let mut sum: f32 = 0 as f32;
    for i in 0..point1.len() {
        sum += (point1[i] - point2[i]) * (point1[i] - point2[i]);
    }
    sum = sum.sqrt();
    sum
}

/*
fn init() -> (i32, i32, i32, Vec<Vec<f32>>) {
    println!("Enter K value (number of clusters) :");
    let num_clusters = input_i32();
    println!("Enter the number of features :");
    let num_feat = input_i32();
    println!("Enter the number of points :");
    let num_points = input_i32();
    let feat = vec![vec![0 as f32; num_feat as usize]; num_points as usize];
    let feat = get_feat(&num_feat, &num_points, feat);
    (num_clusters, num_feat, num_points, feat)
}
*/

fn hardcode_init() -> (i32, i32, Vec<Vec<f32>>) {
    let num_points = 10;
    let _num_feat = 2;
    let num_clusters = 2;
    let feat = vec![
        vec![1.7, 1.8],
        vec![2.6, 1.3],
        vec![4.4, 3.7],
        vec![5.8, 4.1],
        vec![3.6, 2.8],
        vec![2.2, 5.7],
        vec![1.5, 3.3],
        vec![5.1, 1.9],
        vec![4.9, 2.7],
        vec![1.3, 4.5],
    ];
    (num_clusters, num_points, feat)
}

/*
fn get_feat(num_feat: &i32, num_points: &i32, mut feat: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    for i in 0..*num_points {
        println!("Enter data point {}", i + 1);
        for j in 0..*num_feat {
            feat[i as usize][j as usize] = input_f32();
        }
    }
    feat
}
*/

fn init_cluster_points(
    num_clusters: &i32,
    num_points: &i32,
    feat: &Vec<Vec<f32>>,
) -> Vec<Vec<f32>> {
    let mut rng = rand::thread_rng();
    let mut rand_nums = Vec::<i32>::new();
    for _i in 0..*num_clusters {
        let mut num = rng.gen_range(0..*num_points);
        while rand_nums.contains(&num) {
            num = rng.gen_range(0..*num_points);
        }
        rand_nums.push(num);
    }
    let mut cluster_points = Vec::<Vec<f32>>::new();
    for i in 0..*num_clusters {
        let index = rand_nums[i as usize] as usize;
        let temp = &feat[index];
        cluster_points.push(temp.to_vec());
    }
    cluster_points
}

/*
fn input_i32() -> i32 {
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Read error");
    let num = match input.trim().parse::<i32>() {
        Ok(num) => num,
        _ => 0,
    };
    num
}
*/

/*
fn input_f32() -> f32 {
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Read error");
    let num = match input.trim().parse::<f32>() {
        Ok(num) => num,
        _ => 0 as f32,
    };
    num
}
*/
