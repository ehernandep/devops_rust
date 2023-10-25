pipeline {
  agent any
  environment {
    PATH = "/home/ec2-user/.cargo/bin/cargo:$PATH"
  }
  stages {
    stage('verify Cargo installation') {
      steps {
        sh '/home/ec2-user/.cargo/bin/cargo --version'
      }
    }
    stage('compile') {
      steps {
        sh '/home/ec2-user/.cargo/bin/cargo build'
      }
    }
    stage('run with Cargo') {
      steps {
        sh '/home/ec2-user/.cargo/bin/cargo run'
      }
    }
  }
}
