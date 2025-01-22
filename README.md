# 🚀 wave-api
Wave Testing Process

## Pre-requirements
You'll need `rust language` to run this project locally.
See how to install it here: [Rust Get Started page](https://www.rust-lang.org/learn/get-started)

## 🔧 Building and Testing
In the Project directory, you can run:

### development mode
> cargo run dev

### release mode
> cargo build --release && cargo run --release

<br/>

### Database
I am using MongoDB to store the csv data. You can pull and run `mongodb/mongodb-community-server:latest`

``` shell
 docker pull mongodb/mongodb-community-server
 docker run --name mongodb -p 27017:27017 -d mongodb/mongodb-community-server:latest
```
And you should be ready to go.

Database Name: payroll <br/>
Collection Name: employeeReports <br/>
Collection Name: reportFiles <br/>

#### Curl Example for csv Request

``` curl-config
curl --location 'http://127.0.0.1:8080/payroll/time-report/42' \
--form 'file=@"/home/cod3rkane/Downloads/time-report-42.csv"' \
```
