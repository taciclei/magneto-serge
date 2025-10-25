package com.magneto.junit;

import com.magneto.MagnetoProxy;
import com.magneto.ProxyMode;
import org.junit.jupiter.api.extension.*;

import java.lang.annotation.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

/**
 * Extension JUnit 5 pour magneto-serge
 *
 * Enregistrement et rejeu automatique des interactions HTTP/WebSocket dans les tests JUnit.
 *
 * Usage:
 * <pre>
 * {@code
 * @ExtendWith(MagnetoExtension.class)
 * @Magneto(cassette = "api-test")
 * class ApiTest {
 *     @Test
 *     void testApi(MagnetoProxy magneto) {
 *         // Votre test ici
 *     }
 * }
 * }
 * </pre>
 */
public class MagnetoExtension implements
        BeforeAllCallback,
        AfterAllCallback,
        BeforeEachCallback,
        AfterEachCallback,
        ParameterResolver {

    private static final String MAGNETO_KEY = "magneto";
    private static final String DEFAULT_CASSETTE_DIR = "./test_cassettes";
    private static final int DEFAULT_PORT = 8888;

    /**
     * Annotation pour configurer magneto sur une classe ou méthode de test
     */
    @Target({ElementType.TYPE, ElementType.METHOD})
    @Retention(RetentionPolicy.RUNTIME)
    public @interface Magneto {
        /**
         * Nom de la cassette (optionnel)
         */
        String cassette() default "";

        /**
         * Mode proxy: auto, record, replay, strict
         */
        String mode() default "auto";

        /**
         * Répertoire des cassettes
         */
        String cassetteDir() default DEFAULT_CASSETTE_DIR;

        /**
         * Port du proxy
         */
        int port() default DEFAULT_PORT;

        /**
         * Scope: class (partagé) ou method (isolé)
         */
        Scope scope() default Scope.METHOD;
    }

    /**
     * Scope de l'extension
     */
    public enum Scope {
        CLASS,  // Proxy partagé pour toute la classe
        METHOD  // Proxy isolé par méthode de test
    }

    @Override
    public void beforeAll(ExtensionContext context) throws Exception {
        Magneto annotation = findAnnotation(context);
        if (annotation != null && annotation.scope() == Scope.CLASS) {
            MagnetoProxy proxy = createProxy(context, annotation);
            context.getStore(ExtensionContext.Namespace.GLOBAL)
                    .put(MAGNETO_KEY, proxy);
        }
    }

    @Override
    public void afterAll(ExtensionContext context) {
        if (hasClassScopeProxy(context)) {
            MagnetoProxy proxy = getProxy(context);
            if (proxy != null) {
                proxy.shutdown();
            }
        }
    }

    @Override
    public void beforeEach(ExtensionContext context) throws Exception {
        Magneto annotation = findAnnotation(context);
        if (annotation != null && annotation.scope() == Scope.METHOD) {
            MagnetoProxy proxy = createProxy(context, annotation);
            context.getStore(ExtensionContext.Namespace.GLOBAL)
                    .put(MAGNETO_KEY, proxy);
        }
    }

    @Override
    public void afterEach(ExtensionContext context) {
        if (hasMethodScopeProxy(context)) {
            MagnetoProxy proxy = getProxy(context);
            if (proxy != null) {
                proxy.shutdown();
            }
        }
    }

    @Override
    public boolean supportsParameter(ParameterContext parameterContext, ExtensionContext extensionContext) {
        return parameterContext.getParameter().getType().equals(MagnetoProxy.class);
    }

    @Override
    public Object resolveParameter(ParameterContext parameterContext, ExtensionContext extensionContext) {
        return getProxy(extensionContext);
    }

    /**
     * Trouve l'annotation @Magneto sur la méthode ou la classe
     */
    private Magneto findAnnotation(ExtensionContext context) {
        // Chercher sur la méthode d'abord
        Magneto methodAnnotation = context.getTestMethod()
                .map(method -> method.getAnnotation(Magneto.class))
                .orElse(null);

        if (methodAnnotation != null) {
            return methodAnnotation;
        }

        // Sinon chercher sur la classe
        return context.getTestClass()
                .map(clazz -> clazz.getAnnotation(Magneto.class))
                .orElse(null);
    }

    /**
     * Crée une instance MagnetoProxy configurée
     */
    private MagnetoProxy createProxy(ExtensionContext context, Magneto annotation) throws Exception {
        String cassetteDir = getEnvOrDefault("MAGNETO_CASSETTE_DIR", annotation.cassetteDir());
        String mode = getEnvOrDefault("MAGNETO_MODE", annotation.mode());
        int port = Integer.parseInt(getEnvOrDefault("MAGNETO_PORT", String.valueOf(annotation.port())));

        // Créer le répertoire si nécessaire
        Path cassettePath = Paths.get(cassetteDir);
        if (!Files.exists(cassettePath)) {
            Files.createDirectories(cassettePath);
        }

        // Créer le proxy
        MagnetoProxy proxy = new MagnetoProxy(cassetteDir);
        proxy.setPort(port);

        // Déterminer le nom de la cassette
        String cassetteName = determineCassetteName(context, annotation);

        // Configurer le mode
        configureMode(proxy, mode, cassetteName);

        return proxy;
    }

    /**
     * Détermine le nom de la cassette
     */
    private String determineCassetteName(ExtensionContext context, Magneto annotation) {
        if (!annotation.cassette().isEmpty()) {
            return annotation.cassette();
        }

        // Générer depuis le nom du test
        String className = context.getTestClass()
                .map(Class::getSimpleName)
                .orElse("unknown");
        String methodName = context.getTestMethod()
                .map(java.lang.reflect.Method::getName)
                .orElse("unknown");

        return className + "-" + methodName;
    }

    /**
     * Configure le mode du proxy
     */
    private void configureMode(MagnetoProxy proxy, String mode, String cassetteName) {
        switch (mode.toLowerCase()) {
            case "record":
                proxy.setMode(ProxyMode.RECORD);
                proxy.startRecording(cassetteName);
                break;
            case "replay":
                proxy.setMode(ProxyMode.REPLAY);
                proxy.replay(cassetteName);
                break;
            case "strict":
                proxy.replayStrict(cassetteName);
                break;
            case "auto":
            default:
                proxy.hybrid(cassetteName);
                break;
        }
    }

    /**
     * Récupère le proxy depuis le contexte
     */
    private MagnetoProxy getProxy(ExtensionContext context) {
        return context.getStore(ExtensionContext.Namespace.GLOBAL)
                .get(MAGNETO_KEY, MagnetoProxy.class);
    }

    /**
     * Vérifie si un proxy de scope classe existe
     */
    private boolean hasClassScopeProxy(ExtensionContext context) {
        Magneto annotation = findAnnotation(context);
        return annotation != null && annotation.scope() == Scope.CLASS;
    }

    /**
     * Vérifie si un proxy de scope méthode existe
     */
    private boolean hasMethodScopeProxy(ExtensionContext context) {
        Magneto annotation = findAnnotation(context);
        return annotation != null && annotation.scope() == Scope.METHOD;
    }

    /**
     * Récupère une variable d'environnement ou une valeur par défaut
     */
    private String getEnvOrDefault(String key, String defaultValue) {
        String value = System.getenv(key);
        return value != null ? value : defaultValue;
    }

    /**
     * Helper pour configurer le proxy système
     */
    public static void configureSystemProxy(MagnetoProxy proxy) {
        int port = proxy.port();
        System.setProperty("http.proxyHost", "localhost");
        System.setProperty("http.proxyPort", String.valueOf(port));
        System.setProperty("https.proxyHost", "localhost");
        System.setProperty("https.proxyPort", String.valueOf(port));
    }

    /**
     * Helper pour réinitialiser le proxy système
     */
    public static void clearSystemProxy() {
        System.clearProperty("http.proxyHost");
        System.clearProperty("http.proxyPort");
        System.clearProperty("https.proxyHost");
        System.clearProperty("https.proxyPort");
    }
}
