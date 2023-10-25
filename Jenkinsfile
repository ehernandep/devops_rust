pipeline {
  agent any
  stages {
    stage('verify Cargo installation') {
      steps {
        sh '~/.cargo/bin/cargo --version'
      }
    }
    stage('compile') {
      steps {
        sh '~/.cargo/bin/cargo build'
      }
    }
    stage('run with Cargo') {
      steps {
        sh '~/.cargo/bin/cargo run'
      }
    }
  }
}
