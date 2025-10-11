plugins {
    kotlin("jvm") version "1.9.20"
    `java-library`
    `maven-publish`
    signing
}

group = "io.github.magneto"
version = "0.0.1"

kotlin {
    jvmToolchain(11)
}

repositories {
    mavenCentral()
}

dependencies {
    // JNA pour l'interfaçage natif (requis par UniFFI)
    api("net.java.dev.jna:jna:5.13.0")

    // Kotlin standard library
    implementation(kotlin("stdlib"))

    // Test dependencies
    testImplementation(kotlin("test"))
    testImplementation("org.junit.jupiter:junit-jupiter-api:5.10.0")
    testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine:5.10.0")
}

tasks.test {
    useJUnitPlatform()
}

// Configuration du JAR
tasks.jar {
    manifest {
        attributes(
            mapOf(
                "Implementation-Title" to "magneto-serge Kotlin Bindings",
                "Implementation-Version" to version,
                "Implementation-Vendor" to "Magnéto-Serge contributors"
            )
        )
    }
}

// Créer JAR avec sources
val sourcesJar by tasks.registering(Jar::class) {
    archiveClassifier.set("sources")
    from(sourceSets.main.get().allSource)
}

// Créer JAR avec documentation
val javadocJar by tasks.registering(Jar::class) {
    archiveClassifier.set("javadoc")
    from(tasks.dokkaJavadoc.flatMap { it.outputDirectory })
}

// Configuration de publication Maven
publishing {
    publications {
        create<MavenPublication>("mavenKotlin") {
            from(components["java"])
            artifact(sourcesJar)
            artifact(javadocJar)

            pom {
                name.set("magneto-serge-kotlin")
                description.set("Magnéto-Serge Kotlin Bindings - HTTP/WebSocket testing library with record/replay")
                url.set("https://github.com/taciclei/magneto-serge")

                licenses {
                    license {
                        name.set("MIT License")
                        url.set("https://opensource.org/licenses/MIT")
                    }
                    license {
                        name.set("Apache License, Version 2.0")
                        url.set("https://www.apache.org/licenses/LICENSE-2.0.txt")
                    }
                }

                developers {
                    developer {
                        name.set("Magnéto-Serge contributors")
                        email.set("noreply@github.com")
                        organization.set("taciclei")
                        organizationUrl.set("https://github.com/taciclei")
                    }
                }

                scm {
                    connection.set("scm:git:git://github.com/taciclei/magneto-serge.git")
                    developerConnection.set("scm:git:ssh://github.com:taciclei/magneto-serge.git")
                    url.set("https://github.com/taciclei/magneto-serge")
                }
            }
        }
    }

    repositories {
        maven {
            name = "OSSRH"
            val releasesRepoUrl = uri("https://s01.oss.sonatype.org/service/local/staging/deploy/maven2/")
            val snapshotsRepoUrl = uri("https://s01.oss.sonatype.org/content/repositories/snapshots/")
            url = if (version.toString().endsWith("SNAPSHOT")) snapshotsRepoUrl else releasesRepoUrl

            credentials {
                username = System.getenv("OSSRH_USERNAME") ?: project.findProperty("ossrhUsername") as String?
                password = System.getenv("OSSRH_PASSWORD") ?: project.findProperty("ossrhPassword") as String?
            }
        }
    }
}

// Signature des artefacts
signing {
    val signingKey = System.getenv("GPG_PRIVATE_KEY")
    val signingPassword = System.getenv("GPG_PASSPHRASE")

    if (signingKey != null && signingPassword != null) {
        useInMemoryPgpKeys(signingKey, signingPassword)
        sign(publishing.publications["mavenKotlin"])
    }
}

// Wrapper Gradle
tasks.wrapper {
    gradleVersion = "8.5"
    distributionType = Wrapper.DistributionType.BIN
}
