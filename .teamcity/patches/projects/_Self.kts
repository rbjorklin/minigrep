package patches.projects

import jetbrains.buildServer.configs.kotlin.v2018_2.*
import jetbrains.buildServer.configs.kotlin.v2018_2.Project
import jetbrains.buildServer.configs.kotlin.v2018_2.ui.*

/*
This patch script was generated by TeamCity on settings change in UI.
To apply the patch, change the root project
accordingly, and delete the patch script.
*/
changeProject(DslContext.projectId) {
    features {
        add {
            feature {
                type = "OAuthProvider"
                id = "PROJECT_EXT_3"
                param("clientId", "970b81e33fe90be0316a")
                param("defaultTokenScope", "public_repo,repo,repo:status,write:repo_hook")
                param("secure:clientSecret", "credentialsJSON:e9fadfb8-083c-4818-9611-314e5767659e")
                param("displayName", "GitHub.com")
                param("gitHubUrl", "https://github.com/")
                param("providerType", "GitHub")
            }
        }
    }
}
