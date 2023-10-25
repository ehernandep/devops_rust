pipeline {
  agent any
  environment {
    PATH = "/home/ec2-user/.cargo/bin:$PATH"
  }
  stages {
    stage('Set Rust Default Toolchain') {
      steps {
        sh '/root/.cargo/bin/rustup default stable'
      }
    }
    stage('Verify Cargo Installation') {
      steps {
        sh '/home/ec2-user/.cargo/bin/cargo --version'
      }
    }
    stage('Compile') {
      steps {
        sh '/home/ec2-user/.cargo/bin/cargo build'
      }
    }
    stage('Run with Cargo') {
      steps {
        sh '/home/ec2-user/.cargo/bin/cargo run'
      }
    }
  }
}