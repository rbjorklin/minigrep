package patches.buildTypes

import jetbrains.buildServer.configs.kotlin.v2018_2.*
import jetbrains.buildServer.configs.kotlin.v2018_2.buildSteps.ScriptBuildStep
import jetbrains.buildServer.configs.kotlin.v2018_2.buildSteps.dockerCommand
import jetbrains.buildServer.configs.kotlin.v2018_2.buildSteps.script
import jetbrains.buildServer.configs.kotlin.v2018_2.ui.*

/*
This patch script was generated by TeamCity on settings change in UI.
To apply the patch, change the buildType with id = 'Build'
accordingly, and delete the patch script.
*/
changeBuildType(RelativeId("Build")) {
    expectSteps {
        step {
            type = "cargo"
            param("cargo-command", "build")
        }
        step {
            type = "cargo"
            param("cargo-command", "test")
        }
    }
    steps {
        insert(0) {
            script {
                scriptContent = """
                    docker run --rm --volume ${'$'}(pwd):/opt/build --workdir /opt/build rust:slim cargo build --release
                    
                    docker run --rm --volume ${'$'}(pwd):/opt/build --workdir /opt/build rust:slim cargo test
                """.trimIndent()
            }
        }
        insert(1) {
            dockerCommand {
                name = "Build container image"
                commandType = build {
                    source = path {
                        path = "Dockerfile"
                    }
                    commandArgs = "--pull"
                }
                param("dockerImage.platform", "linux")
            }
        }
        insert(2) {
            script {
                name = "test"
                scriptContent = """
                    #!/bin/ash
                    
                    echo hej
                """.trimIndent()
                dockerImage = "alpine:latest"
                dockerImagePlatform = ScriptBuildStep.ImagePlatform.Linux
                dockerPull = true
            }
        }
        items.removeAt(3)
        items.removeAt(3)
    }
}
