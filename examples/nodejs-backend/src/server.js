/**
 * Backend Node.js/Express pour Magneto-Serge avec Alcaeus
 *
 * Cette approche est RECOMMANDÉE pour la production car:
 * - Alcaeus fonctionne nativement dans Node.js (pas de polyfills)
 * - Performances optimales
 * - Cache côté serveur partagé entre clients
 * - Pas de problèmes d'import ESM dans le browser
 * - API simplifiée pour les clients frontend
 */

import express from 'express';
import cors from 'cors';
import Alcaeus from 'alcaeus';

const app = express();
const PORT = process.env.PORT || 3000;
const MAGNETO_API_URL = process.env.MAGNETO_API_URL || 'http://localhost:8889';

// Middleware
app.use(cors());
app.use(express.json());

// Cache de ressources Hydra
const resourceCache = new Map();
const CACHE_TTL = 60000; // 1 minute

/**
 * Helper: Charge une ressource Hydra avec cache
 */
async function loadHydraResource(path) {
  const url = path.startsWith('http') ? path : `${MAGNETO_API_URL}${path}`;
  const cacheKey = url;

  // Vérifier le cache
  const cached = resourceCache.get(cacheKey);
  if (cached && Date.now() - cached.timestamp < CACHE_TTL) {
    return cached.resource;
  }

  // Charger la ressource
  try {
    const response = await Alcaeus.loadResource(url);
    const resource = response.root;

    // Mettre en cache
    resourceCache.set(cacheKey, {
      resource,
      timestamp: Date.now()
    });

    return resource;
  } catch (error) {
    console.error(`Error loading resource ${url}:`, error);
    throw error;
  }
}

/**
 * Helper: Extrait les liens Hydra d'une ressource
 */
function extractHydraLinks(resource) {
  const links = [];

  for (const [predicate, values] of Object.entries(resource)) {
    if (predicate === 'hydra:link' || predicate.endsWith('/link')) {
      const linkArray = Array.isArray(values) ? values : [values];
      links.push(...linkArray);
    }
  }

  return links;
}

/**
 * Helper: Extrait les opérations Hydra
 */
function extractOperations(resource) {
  return resource.operations || [];
}

/**
 * Helper: Convertit une ressource Hydra en JSON simplifié
 */
function simplifyResource(resource) {
  const simplified = {
    '@id': resource['@id'],
    '@type': resource['@type'],
    data: {},
    links: extractHydraLinks(resource).map(link => ({
      title: link.title,
      target: link['hydra:target'],
      operations: link['hydra:operation'] || []
    })),
    operations: extractOperations(resource).map(op => ({
      method: op.method,
      expects: op.expects,
      returns: op.returns,
      target: op.target?.['@id'] || op['@id']
    }))
  };

  // Extraire les données métier (propriétés non-Hydra)
  for (const [key, value] of Object.entries(resource)) {
    if (!key.startsWith('@') && !key.includes('hydra') && !key.includes('operations')) {
      simplified.data[key] = value;
    }
  }

  return simplified;
}

// ==================== ROUTES ====================

/**
 * GET / - Découverte de l'API Magneto
 */
app.get('/', async (req, res) => {
  try {
    const resource = await loadHydraResource('/');
    const simplified = simplifyResource(resource);

    res.json({
      success: true,
      api: {
        name: 'Magneto-Serge Hydra Backend',
        version: '1.0.0',
        magnetoApi: MAGNETO_API_URL,
        description: 'Backend Node.js qui wrappe l\'API Hydra de Magneto-Serge via Alcaeus'
      },
      magnetoRoot: simplified,
      availableEndpoints: [
        'GET /',
        'GET /proxy/status',
        'POST /proxy/start',
        'POST /proxy/stop',
        'GET /cassettes',
        'GET /cassettes/:name',
        'DELETE /cassettes/:name',
        'GET /health'
      ]
    });
  } catch (error) {
    res.status(500).json({
      success: false,
      error: error.message
    });
  }
});

/**
 * GET /proxy/status - Statut du proxy Magneto
 */
app.get('/proxy/status', async (req, res) => {
  try {
    const resource = await loadHydraResource('/proxy/status');
    const simplified = simplifyResource(resource);

    res.json({
      success: true,
      status: simplified.data,
      nextActions: simplified.links
    });
  } catch (error) {
    res.status(500).json({
      success: false,
      error: error.message
    });
  }
});

/**
 * POST /proxy/start - Démarre le proxy
 */
app.post('/proxy/start', async (req, res) => {
  try {
    const { mode, cassette_name, port, strict } = req.body;

    // Charger la ressource pour obtenir l'opération
    const resource = await loadHydraResource('/proxy/start');
    const operations = extractOperations(resource);
    const startOp = operations.find(op => op.method.toLowerCase() === 'post');

    if (!startOp) {
      return res.status(404).json({
        success: false,
        error: 'Start operation not found'
      });
    }

    // Exécuter l'opération
    const response = await startOp.invoke({
      mode: mode || 'auto',
      cassette_name: cassette_name || 'test',
      port: port || 8888,
      strict: strict || false
    });

    const result = simplifyResource(response.root);

    res.json({
      success: true,
      message: 'Proxy started',
      result: result.data,
      nextActions: result.links
    });
  } catch (error) {
    res.status(500).json({
      success: false,
      error: error.message
    });
  }
});

/**
 * POST /proxy/stop - Arrête le proxy
 */
app.post('/proxy/stop', async (req, res) => {
  try {
    const resource = await loadHydraResource('/proxy/stop');
    const operations = extractOperations(resource);
    const stopOp = operations.find(op => op.method.toLowerCase() === 'post');

    if (!stopOp) {
      return res.status(404).json({
        success: false,
        error: 'Stop operation not found'
      });
    }

    const response = await stopOp.invoke(req.body || {});
    const result = simplifyResource(response.root);

    res.json({
      success: true,
      message: 'Proxy stopped',
      result: result.data
    });
  } catch (error) {
    res.status(500).json({
      success: false,
      error: error.message
    });
  }
});

/**
 * GET /cassettes - Liste des cassettes
 */
app.get('/cassettes', async (req, res) => {
  try {
    const resource = await loadHydraResource('/cassettes');
    const simplified = simplifyResource(resource);

    res.json({
      success: true,
      cassettes: simplified.data.data || [],
      pagination: {
        totalItems: resource['hydra:totalItems'],
        view: resource['hydra:view']
      }
    });
  } catch (error) {
    res.status(500).json({
      success: false,
      error: error.message
    });
  }
});

/**
 * GET /cassettes/:name - Détails d'une cassette
 */
app.get('/cassettes/:name', async (req, res) => {
  try {
    const resource = await loadHydraResource(`/cassettes/${req.params.name}`);
    const simplified = simplifyResource(resource);

    res.json({
      success: true,
      cassette: simplified.data,
      operations: simplified.operations
    });
  } catch (error) {
    res.status(404).json({
      success: false,
      error: error.message
    });
  }
});

/**
 * DELETE /cassettes/:name - Supprime une cassette
 */
app.delete('/cassettes/:name', async (req, res) => {
  try {
    const resource = await loadHydraResource(`/cassettes/${req.params.name}`);
    const operations = extractOperations(resource);
    const deleteOp = operations.find(op => op.method.toLowerCase() === 'delete');

    if (!deleteOp) {
      return res.status(404).json({
        success: false,
        error: 'Delete operation not found'
      });
    }

    await deleteOp.invoke();

    res.json({
      success: true,
      message: `Cassette "${req.params.name}" deleted`
    });
  } catch (error) {
    res.status(500).json({
      success: false,
      error: error.message
    });
  }
});

/**
 * GET /health - Santé de l'API
 */
app.get('/health', async (req, res) => {
  try {
    const resource = await loadHydraResource('/health');
    const simplified = simplifyResource(resource);

    res.json({
      success: true,
      health: simplified.data
    });
  } catch (error) {
    res.status(500).json({
      success: false,
      error: error.message
    });
  }
});

/**
 * GET /cache/stats - Statistiques du cache
 */
app.get('/cache/stats', (req, res) => {
  res.json({
    success: true,
    cache: {
      size: resourceCache.size,
      entries: Array.from(resourceCache.keys())
    }
  });
});

/**
 * DELETE /cache - Vide le cache
 */
app.delete('/cache', (req, res) => {
  resourceCache.clear();
  res.json({
    success: true,
    message: 'Cache cleared'
  });
});

// Démarrage du serveur
app.listen(PORT, () => {
  console.log(`
╔════════════════════════════════════════════════════════════╗
║  🌐 Magneto-Serge Hydra Backend (Node.js/Express)         ║
╚════════════════════════════════════════════════════════════╝

✓ Server running on http://localhost:${PORT}
✓ Magneto API: ${MAGNETO_API_URL}
✓ Using Alcaeus for Hydra/JSON-LD navigation
✓ Resource caching enabled (TTL: ${CACHE_TTL}ms)

Available endpoints:
  GET  /                  - API discovery
  GET  /proxy/status      - Proxy status
  POST /proxy/start       - Start proxy
  POST /proxy/stop        - Stop proxy
  GET  /cassettes         - List cassettes
  GET  /cassettes/:name   - Get cassette
  DELETE /cassettes/:name - Delete cassette
  GET  /health            - Health check
  GET  /cache/stats       - Cache statistics
  DELETE /cache           - Clear cache

Press Ctrl+C to stop
  `);
});
