use std::option::Option;

fn main() {
    let vec: Vec<i8> = vec![1, 2, 3, 4, 5];
    let arg_test: Vec<f32> = vec![1.0, 3.0, -1.0];
    let arg_res = argsort(&arg_test);
    let y_data: Vec<i8> = vec![-1, 1, 1];
    let x_data: Vec<Vec<f32>> = vec![vec![1.0], vec![5.0], vec![6.0]];

    let mean = mean(&vec);
    println!("Mean is {mean}");
    let impure = sqimpurity(&y_data);
    println!("impurity is {impure}");
    println!("Argsort of {:?} is {:?}", arg_test, arg_res);
    let (cut, feat, loss) = sqsplit(&x_data, &y_data);
    println!(
        "SQsplit of random array gives\n Feature: {feat}\nCutoff value: {cut}\nImpurity: {loss}"
    )
}

struct TreeNode {
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
    split_value: Option<f32>,
    split_feature: Option<u8>,
    split_prediction: i8,
}


fn cart_train(x_train: &[[f32]], y_train: &[i8], max_depth: &usize) -> TreeNode {
    let prediction = mean(y_train)

    if prediction == 1.0 {
        leaf = TreeNode{ None, None, None, None, 1}
    }
    else if prediciton == -1.0 {
        leaf = TreeNode{ None, None, None, None, -1}
    }
    else {
        // values to be passed left and right 
        feature, cutoff, loss = sqsplit(&x_train, &y_train); 

        let left_node = cart_train(&left_x, &left_y, max_depth -1);
        let right_node = cart_train(&right_x, &right_y, max_depth -1);

        let node = TreeNode{left_node, right_node, cutoff, feature, prediciton};

    }


}


fn sqsplit(x_data: &Vec<Vec<f32>>, y_data: &Vec<i8>) -> (f32, usize, f32) {
    let mut best_loss: f32 = f32::INFINITY;
    let mut best_cut: f32 = f32::INFINITY;
    let mut best_feature: usize = 255;
    let num_features: usize = x_data[0].len();
    let n: usize = x_data.len();

    for col in 0..num_features {
        let mut x_at_col = Vec::new();

        for datapoint in x_data {
            x_at_col.push(&datapoint[col]);
        }

        let x_at_col = x_at_col; // make immutable.
        let sorted_col_ind = argsort(&x_at_col);
        let x_sort = sort_vec_by_ind(&x_at_col, &sorted_col_ind);
        let y_sort = sort_vec_by_ind(&y_data, &sorted_col_ind);

        println!("{:?}", x_sort);
        println!("{:?}", y_sort);

        for idx in 0..n - 1 {

            if x_sort[idx] < x_sort[idx + 1] {
                

                let y_left = &y_sort[0..idx];
                let y_right = &y_sort[idx..];

                let loss = sqimpurity(&y_left) + sqimpurity(&y_right);

                if loss < best_loss {
                    println!("Gets Here");
                    best_loss = loss;
                    best_cut = (x_sort[idx-1] + x_sort[idx]) / 2.0;
                    best_feature = col;
                }
            }
        }
    }
    return (best_cut, best_feature, best_loss);
}

fn sort_vec_by_ind<T: Copy>(vec: &Vec<T>, indeces: &Vec<usize>) -> Vec<T> {
    let mut result_vec = Vec::new();
    for idx in indeces {
        result_vec.push(vec[*idx]);
    }
    return result_vec;
}

fn sqimpurity(y_data: &[i8]) -> f32 {
    let mut impurity: f32 = 0.0;
    let mean: f32 = mean(&y_data);

    for y in y_data {
        impurity = (mean - *y as f32).powf(2.0);
    }
    impurity = impurity / y_data.len() as f32;

    return mean;
}

fn mean(vector: &[i8]) -> f32 {
    let mut mean: f32 = 0.0;
    for value in vector {
        mean += *value as f32;
    }
    mean = mean / vector.len() as f32;

    return mean;
}
fn argsort<T: PartialOrd>(data: &[T]) -> Vec<usize> {
    let mut indeces = (0..data.len()).collect::<Vec<_>>();
    // Sort by takes a closure that defines the ordering of sort_by
    // sort by key takes a closure that defines the magnitude of each element.
    indeces.sort_by(|&a, &b| data[a].partial_cmp(&data[b]).unwrap());
    return indeces;
}
