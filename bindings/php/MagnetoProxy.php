<?php
/**
 * MagnetoProxy - PHP bindings for matgto-serge
 *
 * This class provides PHP bindings to the Rust-based matgto-serge library
 * using PHP's FFI (Foreign Function Interface).
 *
 * @package MatgtoSerge
 * @version 0.2.0
 * @license MIT OR Apache-2.0
 */

namespace MatgtoSerge;

use FFI;
use Exception;

/**
 * Proxy operating modes
 */
enum ProxyMode: string {
    case Auto = 'auto';
    case Record = 'record';
    case Replay = 'replay';
    case Passthrough = 'passthrough';
}

/**
 * Main MagnetoProxy class
 */
class MagnetoProxy {
    private FFI $ffi;
    private mixed $proxy_ptr;
    private string $cassette_dir;
    private int $port = 8888;
    private ProxyMode $mode = ProxyMode::Auto;

    /**
     * Initialize FFI and load the Rust library
     */
    public function __construct(string $cassette_dir) {
        $this->cassette_dir = $cassette_dir;

        // Determine library path based on OS
        $lib_path = $this->findLibraryPath();

        if (!file_exists($lib_path)) {
            throw new Exception("matgto-serge library not found at: $lib_path. Please build the library first.");
        }

        // Define FFI interface
        $this->ffi = FFI::cdef("
            // Opaque pointer types
            typedef struct MagnetoProxy MagnetoProxy;

            // Factory function
            MagnetoProxy* matgto_create_proxy(const char* cassette_dir);

            // Configuration
            void matgto_proxy_set_port(MagnetoProxy* proxy, uint16_t port);
            void matgto_proxy_set_mode(MagnetoProxy* proxy, const char* mode);

            // Recording
            int matgto_start_recording(MagnetoProxy* proxy, const char* cassette_name);
            int matgto_stop_recording(MagnetoProxy* proxy);

            // Replay
            int matgto_replay(MagnetoProxy* proxy, const char* cassette_name);

            // Getters
            uint16_t matgto_proxy_get_port(MagnetoProxy* proxy);
            const char* matgto_proxy_get_mode(MagnetoProxy* proxy);

            // Cleanup
            void matgto_proxy_shutdown(MagnetoProxy* proxy);
            void matgto_proxy_free(MagnetoProxy* proxy);
        ", $lib_path);

        // Create proxy instance
        $this->proxy_ptr = $this->ffi->matgto_create_proxy($cassette_dir);

        if ($this->proxy_ptr === null) {
            throw new Exception("Failed to create MagnetoProxy instance");
        }
    }

    /**
     * Find the library path based on OS
     */
    private function findLibraryPath(): string {
        $base_path = dirname(__DIR__, 2) . '/target/release/';

        if (PHP_OS_FAMILY === 'Darwin') {
            return $base_path . 'libmatgto_serge.dylib';
        } elseif (PHP_OS_FAMILY === 'Windows') {
            return $base_path . 'matgto_serge.dll';
        } else {
            return $base_path . 'libmatgto_serge.so';
        }
    }

    /**
     * Set proxy port
     */
    public function withPort(int $port): self {
        $this->port = $port;
        $this->ffi->matgto_proxy_set_port($this->proxy_ptr, $port);
        return $this;
    }

    /**
     * Set proxy mode
     */
    public function withMode(ProxyMode $mode): self {
        $this->mode = $mode;
        $this->ffi->matgto_proxy_set_mode($this->proxy_ptr, $mode->value);
        return $this;
    }

    /**
     * Start recording to a cassette
     */
    public function startRecording(string $cassetteName): void {
        $result = $this->ffi->matgto_start_recording($this->proxy_ptr, $cassetteName);

        if ($result !== 0) {
            throw new Exception("Failed to start recording: error code $result");
        }
    }

    /**
     * Stop recording
     */
    public function stopRecording(): void {
        $result = $this->ffi->matgto_stop_recording($this->proxy_ptr);

        if ($result !== 0) {
            throw new Exception("Failed to stop recording: error code $result");
        }
    }

    /**
     * Replay a cassette
     */
    public function replay(string $cassetteName): void {
        $result = $this->ffi->matgto_replay($this->proxy_ptr, $cassetteName);

        if ($result !== 0) {
            throw new Exception("Failed to replay cassette: error code $result");
        }
    }

    /**
     * Get current port
     */
    public function getPort(): int {
        return $this->ffi->matgto_proxy_get_port($this->proxy_ptr);
    }

    /**
     * Get current mode
     */
    public function getMode(): ProxyMode {
        $mode_str = $this->ffi->matgto_proxy_get_mode($this->proxy_ptr);
        return ProxyMode::from($mode_str);
    }

    /**
     * Shutdown the proxy
     */
    public function shutdown(): void {
        $this->ffi->matgto_proxy_shutdown($this->proxy_ptr);
    }

    /**
     * Cleanup
     */
    public function __destruct() {
        if ($this->proxy_ptr !== null) {
            $this->ffi->matgto_proxy_free($this->proxy_ptr);
        }
    }
}

/**
 * Factory function for convenience
 */
function createProxy(string $cassetteDir): MagnetoProxy {
    return new MagnetoProxy($cassetteDir);
}
