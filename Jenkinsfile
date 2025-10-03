// Jenkinsfile for the furrk-rust-bot (GitFlow Ready)

pipeline {
    agent any

    environment {
        DOCKERHUB_CREDENTIALS = 'DOCKERHUB_CREDENTIALS'
        DOCKER_USERNAME = 'ZuriTheChee'
        // --- CHANGE: The image name for this service ---
        IMAGE_NAME = 'furrk-rust-bot'
    }

    stages {
        // --- Stage 1: Build & Test Feature Branches ---
        stage('Build & Test Feature') {
            when {
                branch 'feature/*'
            }
            steps {
                script {
                    echo "Building and testing branch: ${env.BRANCH_NAME}"
                    docker.image('rust:1.79-slim').inside {
                        sh 'cargo build --release'
                        sh 'cargo test --release'
                    }
                    echo "Build and tests passed."
                }
            }
        }

        // --- Stage 2: Publish & Deploy Develop Branch ---
        stage('Publish & Deploy Develop') {
            when {
                branch '*/develop'
            }
            steps {
                script {
                    echo "Building image for 'develop' branch..."
                    withCredentials([usernamePassword(credentialsId: DOCKERHUB_CREDENTIALS, usernameVariable: 'DOCKER_USER', passwordVariable: 'DOCKER_PASS')]) {
                        sh "docker login -u ${DOCKER_USER} -p ${DOCKER_PASS}"
                    }
                    def fullImageName = "${DOCKER_USERNAME}/${IMAGE_NAME}:latest"
                    sh "docker build -t ${fullImageName} ."
                    sh "docker push ${fullImageName}"
                    echo "Pushed Development Image: ${fullImageName}"

                    echo "Deploying to local Docker instance..."
                    sh """
                        cd /home/yagdrassyl/rust-services && \\
                        docker-compose -p rust_dev -f docker-compose.dev.yml pull telegram-bot && \\
                        docker-compose -p rust_dev -f docker-compose.dev.yml up -d --no-deps telegram-bot
                    """
                }
            }
        }
    }

    post {
        always {
            sh 'if [ -f ~/.docker/config.json ]; then docker logout; fi'
        }
    }
}
