pipeline {
  agent any
  environment {
    PATH = "/home/ec2-user/.cargo/bin:$PATH"
  }
  stages {
    stage('Set Rust Default Toolchain') {
      steps {
        sh 'sudo /root/.cargo/bin/rustup default stable'
      }
    }
    stage('Verify Cargo Installation') {
      steps {
        sh 'sudo /home/ec2-user/.cargo/bin/cargo --version'
      }
    }
    stage('Compile') {
      steps {
        sh 'sudo /home/ec2-user/.cargo/bin/cargo build'
      }
    }
    stage('Run with Cargo') {
      steps {
        sh 'sudo /home/ec2-user/.cargo/bin/cargo run'
      }
    }
  }
}