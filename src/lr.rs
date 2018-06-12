// input values X
//     are combined linearly using weight to predict an output y

//     yhat = 1.0 / (1.0 + e ^ (-(b0 + b1 * x1))
                  
// e is the base of natulra logarithms
// yhat is the predicted output
// b0 is the bias
// b1 is the coefficient for the single input value x1
// yhat is a value between 0 and 1 and that needs to be rounded and mapped to the predicted class

// each column in your input data has an associated b coefficient tha tmust be learned from the training data


// Stochastic gradient descent
// minif a function by following the gradients of the cost function

// b = b + b learning_rate * (y - yhat) * yhat * (1 - yhat) * x
// (y - yhat) is the prediction error for the model
// yhat is the prediction made by the coefficients
// x is input

fn predict(row: Vec<f64>, coefficients: Vec<f64>) -> f64 {
    let mut yhat = coefficients[0];
    for i in 0..(row.len() - 1) {
        yhat += coefficients[i + 1] * row[i];
    }
    1.0 / (1.0 + (-yhat).exp())
}

fn coefficients_sgd(train: Vec<Vec<f64>>, l_rate, n_epoch) -> f64 {
    let mut coef = [0.0; train[0].len());
    for epoch in 0..n_epoch {
        let mut sum_error = 0.0;
        for row in train {
            let yhat = predict(row, coef);
            let error = row[-1] - yhat;
            sum_error += error**2;
            coef[0] = coef[0] + l_rate * error * yhat * (1.0 - yhat);
            for i in 0..(row.len() - 1) {
                coef[i+1] = coef[i+1] + l_rate * error * yhat * (1.0 - yhat) * row[i]; 
            }
        }
    }
    coef
}
