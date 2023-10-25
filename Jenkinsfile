pipeline {
  agent any
  stages {
    stage('verify Cargo installation') {
      steps {
        sh 'cargo --version'
      }
    }
    stage('compile') {
      steps {
        sh 'cargo build'
      }
    }
    stage('run with Cargo') {
      steps {
        sh 'cargo run'
      }
    }
  }
}