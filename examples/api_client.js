#!/usr/bin/env node
/**
 * Example Node.js client for Magneto-Serge REST API
 *
 * Demonstrates how to:
 * - Start/stop the proxy
 * - Check proxy status
 * - List cassettes
 * - Navigate using Hydra links
 */

class MagnetoAPI {
    constructor(baseUrl = 'http://localhost:8889', apiKey = null) {
        this.baseUrl = baseUrl.replace(/\/$/, '');
        this.apiKey = apiKey;
    }

    async _request(method, path, body = null) {
        const url = `${this.baseUrl}${path}`;
        const options = {
            method,
            headers: {
                'Content-Type': 'application/json',
            },
        };

        if (this.apiKey) {
            options.headers['Authorization'] = `Bearer ${this.apiKey}`;
        }

        if (body) {
            options.body = JSON.stringify(body);
        }

        const response = await fetch(url, options);

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.error || `HTTP ${response.status}`);
        }

        return response.json();
    }

    async getRoot() {
        return this._request('GET', '/');
    }

    async health() {
        return this._request('GET', '/health');
    }

    async startProxy(cassetteName, { mode = 'auto', port = null, strict = false } = {}) {
        const data = {
            mode,
            cassette_name: cassetteName,
            strict,
        };
        if (port) data.port = port;
        return this._request('POST', '/proxy/start', data);
    }

    async stopProxy(force = false) {
        return this._request('POST', '/proxy/stop', { force });
    }

    async getStatus() {
        return this._request('GET', '/proxy/status');
    }

    async getStats() {
        return this._request('GET', '/proxy/stats');
    }

    async listCassettes() {
        return this._request('GET', '/cassettes');
    }

    async getCassette(name) {
        return this._request('GET', `/cassettes/${name}`);
    }

    async deleteCassette(name) {
        return this._request('DELETE', `/cassettes/${name}`);
    }

    async getOpenAPISpec() {
        return this._request('GET', '/openapi.json');
    }

    async followLink(response, linkTitle) {
        const links = response['hydra:link'] || [];
        const link = links.find(l => l.title === linkTitle);

        if (!link) {
            throw new Error(`Link with title '${linkTitle}' not found`);
        }

        const target = link['hydra:target'];
        const path = target.replace(this.baseUrl, '');
        return this._request('GET', path);
    }
}

async function main() {
    const api = new MagnetoAPI();

    console.log('🌐 Magneto-Serge API Client Example\n');

    try {
        // 1. Get API root and discover endpoints
        console.log('1️⃣  Getting API root...');
        const root = await api.getRoot();
        console.log(`   API: ${root.data.title}`);
        console.log(`   Version: ${root.data.version}`);
        console.log(`   Available links: ${(root['hydra:link'] || []).length}`);
        console.log();

        // 2. Check health
        console.log('2️⃣  Checking health...');
        const health = await api.health();
        console.log(`   Status: ${health.data.status}`);
        console.log(`   Uptime: ${health.data.uptime_seconds} seconds`);
        console.log();

        // 3. Get proxy status
        console.log('3️⃣  Getting proxy status...');
        const status = await api.getStatus();
        console.log(`   Running: ${status.data.running}`);
        console.log(`   Mode: ${status.data.mode}`);
        console.log();

        // 4. Start proxy in auto mode
        console.log('4️⃣  Starting proxy in auto mode...');
        try {
            const startResponse = await api.startProxy('example-test', {
                mode: 'auto',
                port: 8888
            });
            console.log(`   ✓ ${startResponse.data.message}`);
            console.log(`   Mode: ${startResponse.data.mode}`);
            console.log(`   Cassette: ${startResponse.data.cassette}`);
            console.log(`   Port: ${startResponse.data.port}`);

            // Follow Hydra link to check status
            console.log('\n   Following "Check Proxy Status" link...');
            const newStatus = await api.followLink(startResponse, 'Check Proxy Status');
            console.log(`   Proxy running: ${newStatus.data.running}`);

            // Stop proxy
            console.log('\n5️⃣  Stopping proxy...');
            const stopResponse = await api.stopProxy();
            console.log(`   ✓ ${stopResponse.data.message}`);
        } catch (err) {
            if (err.message.includes('409') || err.message.includes('already running')) {
                console.log('   ⚠ Proxy already running');
            } else {
                throw err;
            }
        }
        console.log();

        // 6. List cassettes
        console.log('6️⃣  Listing cassettes...');
        const cassettesResponse = await api.listCassettes();
        const cassettes = cassettesResponse.data;
        console.log(`   Found ${cassettes.length} cassettes:`);
        cassettes.slice(0, 5).forEach(cassette => {
            console.log(`   • ${cassette.name} (${cassette.size_bytes} bytes)`);
        });
        console.log();

        // 7. Get OpenAPI spec
        console.log('7️⃣  Getting OpenAPI specification...');
        const spec = await api.getOpenAPISpec();
        console.log(`   OpenAPI version: ${spec.openapi}`);
        console.log(`   API title: ${spec.info.title}`);
        console.log(`   Endpoints: ${Object.keys(spec.paths).length}`);
        console.log();

        console.log('✅ All operations completed successfully!');

    } catch (error) {
        if (error.cause?.code === 'ECONNREFUSED') {
            console.error('❌ Error: Cannot connect to API server');
            console.error('   Make sure the API server is running:');
            console.error('   $ magneto api');
        } else {
            console.error(`❌ Error: ${error.message}`);
        }
        process.exit(1);
    }
}

// Run if executed directly
if (require.main === module) {
    main();
}

module.exports = { MagnetoAPI };
