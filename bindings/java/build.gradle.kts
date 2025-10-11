plugins {
    java
    `java-library`
    `maven-publish`
    signing
    id("org.jetbrains.kotlin.jvm") version "1.9.20"
}

group = "io.github.magneto"
version = "0.0.1"

java {
    sourceCompatibility = JavaVersion.VERSION_11
    targetCompatibility = JavaVersion.VERSION_11
    withSourcesJar()
    withJavadocJar()
}

repositories {
    mavenCentral()
}

dependencies {
    // Bindings Kotlin générés par UniFFI
    implementation(fileTree(mapOf("dir" to "../kotlin", "include" to listOf("**/*.jar"))))

    // JNA pour l'interfaçage natif (requis par UniFFI/Kotlin)
    implementation("net.java.dev.jna:jna:5.13.0")

    // Kotlin stdlib (requis pour utiliser les bindings Kotlin)
    implementation("org.jetbrains.kotlin:kotlin-stdlib:1.9.20")

    // JUnit 5 pour les tests
    testImplementation("org.junit.jupiter:junit-jupiter-api:5.10.0")
    testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine:5.10.0")
    testImplementation("org.junit.jupiter:junit-jupiter-params:5.10.0")
}

// Configuration pour inclure la bibliothèque native
val copyNativeLib by tasks.registering(Copy::class) {
    from("../kotlin/libuniffi_magneto_serge.dylib", "../kotlin/libuniffi_magneto_serge.so")
    into("build/libs")
}

// S'assurer que la lib native est copiée avant les tests
tasks.test {
    dependsOn(copyNativeLib)
    useJUnitPlatform()

    // Ajouter la bibliothèque native au path
    systemProperty("java.library.path", "build/libs")

    testLogging {
        events("passed", "skipped", "failed")
        exceptionFormat = org.gradle.api.tasks.testing.logging.TestExceptionFormat.FULL
        showStandardStreams = false
    }
}

// Configuration du JAR
tasks.jar {
    manifest {
        attributes(
            mapOf(
                "Implementation-Title" to "magneto-serge Java Bindings",
                "Implementation-Version" to version,
                "Implementation-Vendor" to "Magnéto-Serge contributors"
            )
        )
    }

    // Inclure les dépendances dans le JAR
    from(configurations.runtimeClasspath.get().map { if (it.isDirectory) it else zipTree(it) })

    duplicatesStrategy = DuplicatesStrategy.EXCLUDE
}

// Tâche pour exécuter l'exemple
val runExample by tasks.registering(JavaExec::class) {
    dependsOn(copyNativeLib)

    classpath = sourceSets.main.get().runtimeClasspath
    mainClass.set("io.github.magneto.serge.examples.Example")

    // Ajouter la lib native
    systemProperty("java.library.path", "build/libs")
}

// Configuration Javadoc
tasks.javadoc {
    options {
        encoding = "UTF-8"
        (this as StandardJavadocDocletOptions).apply {
            charSet = "UTF-8"
            author(true)
            version(true)
            links("https://docs.oracle.com/en/java/javase/11/docs/api/")
        }
    }
}

// Configuration de publication Maven
publishing {
    publications {
        create<MavenPublication>("mavenJava") {
            from(components["java"])

            pom {
                name.set("magneto-serge")
                description.set("Magnéto-Serge - Multi-language HTTP/WebSocket testing library with record/replay capabilities")
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
    // Utiliser les variables d'environnement pour GPG
    val signingKey = System.getenv("GPG_PRIVATE_KEY")
    val signingPassword = System.getenv("GPG_PASSPHRASE")

    if (signingKey != null && signingPassword != null) {
        useInMemoryPgpKeys(signingKey, signingPassword)
        sign(publishing.publications["mavenJava"])
    }
}

// Wrapper Gradle
tasks.wrapper {
    gradleVersion = "8.5"
    distributionType = Wrapper.DistributionType.BIN
}
