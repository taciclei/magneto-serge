# Makefile pour Magneto-Serge
# Automatise la compilation, les tests, et le démarrage de l'écosystème complet

.PHONY: help install build test clean run-api run-backend run-client-simple run-client-hydra dev stop docker docs

# Couleurs pour les messages
RED    := \033[0;31m
GREEN  := \033[0;32m
YELLOW := \033[0;33m
BLUE   := \033[0;34m
RESET  := \033[0m

# Configuration
CARGO := cargo
NPM := npm
NODE := node
RUST_LOG ?= info

##@ Aide

help: ## Affiche cette aide
	@echo "$(BLUE)Magneto-Serge - Makefile$(RESET)"
	@echo ""
	@awk 'BEGIN {FS = ":.*##"; printf "Usage: make $(YELLOW)<target>$(RESET)\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  $(GREEN)%-20s$(RESET) %s\n", $$1, $$2 } /^##@/ { printf "\n$(BLUE)%s$(RESET)\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Installation

install: install-rust install-backend install-client-simple install-client-hydra ## Installe toutes les dépendances
	@echo "$(GREEN)✓ Toutes les dépendances sont installées$(RESET)"

install-rust: ## Installe les dépendances Rust
	@echo "$(BLUE)Installation des dépendances Rust...$(RESET)"
	@$(CARGO) fetch
	@echo "$(GREEN)✓ Dépendances Rust installées$(RESET)"

install-backend: ## Installe les dépendances du backend Node.js
	@echo "$(BLUE)Installation des dépendances backend Node.js...$(RESET)"
	@cd examples/nodejs-backend && $(NPM) install
	@echo "$(GREEN)✓ Backend Node.js prêt$(RESET)"

install-client-simple: ## Installe les dépendances du client Angular simple
	@echo "$(BLUE)Installation du client Angular simple...$(RESET)"
	@cd examples/angular-simple-client && $(NPM) install
	@echo "$(GREEN)✓ Client Angular simple prêt$(RESET)"

install-client-hydra: ## Installe les dépendances du client Angular Hydra
	@echo "$(BLUE)Installation du client Angular Hydra...$(RESET)"
	@cd examples/angular-client && $(NPM) install
	@echo "$(GREEN)✓ Client Angular Hydra prêt$(RESET)"

##@ Compilation

build: ## Compile le projet Rust (debug)
	@echo "$(BLUE)Compilation du projet Rust...$(RESET)"
	@$(CARGO) build
	@echo "$(GREEN)✓ Compilation terminée$(RESET)"

build-release: ## Compile le projet Rust (release optimisé)
	@echo "$(BLUE)Compilation release...$(RESET)"
	@$(CARGO) build --release
	@echo "$(GREEN)✓ Binaire release créé: target/release/magneto$(RESET)"

build-cli: build-release ## Compile et installe le CLI
	@echo "$(BLUE)Installation du CLI magneto...$(RESET)"
	@$(CARGO) install --path .
	@echo "$(GREEN)✓ CLI installé: magneto$(RESET)"

build-all: build build-client-simple build-client-hydra ## Compile tout (Rust + Angular clients)
	@echo "$(GREEN)✓ Tous les projets compilés$(RESET)"

build-client-simple: ## Compile le client Angular simple
	@echo "$(BLUE)Build du client Angular simple...$(RESET)"
	@cd examples/angular-simple-client && $(NPM) run build
	@echo "$(GREEN)✓ Client simple compilé: examples/angular-simple-client/dist/$(RESET)"

build-client-hydra: ## Compile le client Angular Hydra
	@echo "$(BLUE)Build du client Angular Hydra...$(RESET)"
	@cd examples/angular-client && $(NPM) run build
	@echo "$(GREEN)✓ Client Hydra compilé: examples/angular-client/dist/$(RESET)"

##@ Tests

test: ## Lance tous les tests Rust
	@echo "$(BLUE)Lancement des tests...$(RESET)"
	@$(CARGO) test
	@echo "$(GREEN)✓ Tests réussis$(RESET)"

test-verbose: ## Lance les tests en mode verbose
	@$(CARGO) test -- --nocapture

check: ## Vérifie la compilation sans build complet
	@echo "$(BLUE)Vérification du code...$(RESET)"
	@$(CARGO) check
	@echo "$(GREEN)✓ Code valide$(RESET)"

clippy: ## Lance clippy (linter Rust)
	@echo "$(BLUE)Analyse avec clippy...$(RESET)"
	@$(CARGO) clippy --all-features --all-targets -- -D warnings
	@echo "$(GREEN)✓ Clippy OK$(RESET)"

fmt: ## Formate le code Rust
	@echo "$(BLUE)Formatage du code...$(RESET)"
	@$(CARGO) fmt
	@echo "$(GREEN)✓ Code formaté$(RESET)"

fmt-check: ## Vérifie le formatage sans modifier
	@$(CARGO) fmt -- --check

##@ Démarrage (Services Individuels)

run-api: build ## Démarre l'API Magneto-Serge (port 8889)
	@echo "$(BLUE)Démarrage de l'API Magneto-Serge...$(RESET)"
	@echo "$(YELLOW)API: http://localhost:8889$(RESET)"
	@echo "$(YELLOW)OpenAPI: http://localhost:8889/openapi.json$(RESET)"
	@echo "$(YELLOW)Health: http://localhost:8889/health$(RESET)"
	@RUST_LOG=$(RUST_LOG) $(CARGO) run --bin magneto -- api

run-backend: ## Démarre le backend Node.js (port 3000)
	@echo "$(BLUE)Démarrage du backend Node.js...$(RESET)"
	@echo "$(YELLOW)Backend: http://localhost:3000$(RESET)"
	@cd examples/nodejs-backend && $(NPM) start

run-client-simple: ## Démarre le client Angular simple (port 4201)
	@echo "$(BLUE)Démarrage du client Angular simple...$(RESET)"
	@echo "$(YELLOW)Client: http://localhost:4201$(RESET)"
	@cd examples/angular-simple-client && $(NPM) start

run-client-hydra: ## Démarre le client Angular Hydra (port 4200)
	@echo "$(BLUE)Démarrage du client Angular Hydra...$(RESET)"
	@echo "$(YELLOW)Client: http://localhost:4200$(RESET)"
	@cd examples/angular-client && $(NPM) start

##@ Démarrage (Stack Complète)

dev: ## Lance la stack complète de développement (recommandé)
	@echo "$(BLUE)╔════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(BLUE)║  Démarrage de la stack complète Magneto-Serge         ║$(RESET)"
	@echo "$(BLUE)╚════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@echo "$(YELLOW)Cette commande nécessite plusieurs terminaux.$(RESET)"
	@echo "$(YELLOW)Utilisez 'make dev-tmux' pour un démarrage automatique.$(RESET)"
	@echo ""
	@echo "$(GREEN)Lancement des services:$(RESET)"
	@echo "  1. $(BLUE)API Magneto$(RESET) (port 8889)"
	@echo "  2. $(BLUE)Backend Node.js$(RESET) (port 3000)"
	@echo "  3. $(BLUE)Client Angular$(RESET) (port 4201)"
	@echo ""
	@$(MAKE) dev-tmux

dev-tmux: ## Lance la stack complète dans tmux (automatique)
	@if command -v tmux >/dev/null 2>&1; then \
		echo "$(BLUE)Lancement dans tmux...$(RESET)"; \
		./scripts/start-dev.sh; \
	else \
		echo "$(RED)tmux n'est pas installé. Installation:$(RESET)"; \
		echo "  macOS: brew install tmux"; \
		echo "  Linux: sudo apt install tmux"; \
		echo ""; \
		echo "$(YELLOW)Ou lancez manuellement dans 3 terminaux:$(RESET)"; \
		echo "  Terminal 1: make run-api"; \
		echo "  Terminal 2: make run-backend"; \
		echo "  Terminal 3: make run-client-simple"; \
	fi

dev-manual: ## Instructions pour démarrage manuel
	@echo "$(BLUE)╔════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(BLUE)║  Instructions de démarrage manuel                      ║$(RESET)"
	@echo "$(BLUE)╚════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@echo "$(GREEN)Terminal 1$(RESET) - API Magneto-Serge:"
	@echo "  $$ make run-api"
	@echo ""
	@echo "$(GREEN)Terminal 2$(RESET) - Backend Node.js:"
	@echo "  $$ make run-backend"
	@echo ""
	@echo "$(GREEN)Terminal 3$(RESET) - Client Angular:"
	@echo "  $$ make run-client-simple"
	@echo ""
	@echo "$(YELLOW)Puis ouvrez: http://localhost:4201$(RESET)"

##@ Exemples CLI

example-record: build ## Exemple: enregistrer des requêtes
	@echo "$(BLUE)Démarrage du proxy en mode record...$(RESET)"
	@echo "$(YELLOW)Configurez votre client HTTP:$(RESET)"
	@echo "  export http_proxy=http://localhost:8888"
	@echo "  export https_proxy=http://localhost:8888"
	@echo ""
	@$(CARGO) run --bin magneto -- record demo --port 8888

example-replay: build ## Exemple: rejouer des requêtes
	@echo "$(BLUE)Démarrage du proxy en mode replay...$(RESET)"
	@$(CARGO) run --bin magneto -- replay demo --port 8888

example-auto: build ## Exemple: mode auto (intelligent)
	@echo "$(BLUE)Démarrage du proxy en mode auto...$(RESET)"
	@$(CARGO) run --bin magneto -- auto demo --port 8888

example-list: build ## Exemple: lister les cassettes
	@$(CARGO) run --bin magneto -- list

##@ Docker

docker-build: ## Build l'image Docker standard
	@echo "$(BLUE)Build de l'image Docker...$(RESET)"
	@docker build -t magneto-serge:latest .
	@echo "$(GREEN)✓ Image Docker créée: magneto-serge:latest$(RESET)"

docker-build-alpine: ## Build l'image Docker Alpine (optimisée)
	@echo "$(BLUE)Build de l'image Docker Alpine...$(RESET)"
	@docker build -f Dockerfile.alpine -t magneto-serge:alpine -t tabou/magneto-serge:alpine .
	@echo "$(GREEN)✓ Image Docker Alpine créée: magneto-serge:alpine$(RESET)"

docker-run: ## Lance le container Docker
	@echo "$(BLUE)Démarrage du container Docker...$(RESET)"
	@docker run -p 8889:8889 -p 8888:8888 magneto-serge:latest

docker-run-alpine: ## Lance le container Docker Alpine (tous services)
	@echo "$(BLUE)Démarrage du container Docker Alpine...$(RESET)"
	@docker run -d --name magneto-serge \
		-p 8889:8889 -p 3000:3000 -p 4201:4201 -p 8888:8888 \
		-v $(PWD)/cassettes:/app/cassettes \
		magneto-serge:alpine all
	@echo "$(GREEN)✓ Container démarré$(RESET)"
	@echo "$(YELLOW)API: http://localhost:8889$(RESET)"
	@echo "$(YELLOW)Backend: http://localhost:3000$(RESET)"
	@echo "$(YELLOW)Frontend: http://localhost:4201$(RESET)"
	@echo "$(YELLOW)Proxy: http://localhost:8888$(RESET)"

docker-compose: ## Lance avec docker-compose (stack complète)
	@echo "$(BLUE)Démarrage avec docker-compose...$(RESET)"
	@docker-compose up -d
	@echo "$(GREEN)✓ Stack démarrée$(RESET)"
	@echo "$(YELLOW)API: http://localhost:8889$(RESET)"
	@echo "$(YELLOW)Backend: http://localhost:3000$(RESET)"
	@echo "$(YELLOW)Client: http://localhost:4201$(RESET)"

docker-compose-alpine: ## Lance avec docker-compose Alpine
	@echo "$(BLUE)Démarrage avec docker-compose Alpine...$(RESET)"
	@docker-compose -f docker-compose.alpine.yml up -d
	@echo "$(GREEN)✓ Stack Alpine démarrée$(RESET)"
	@echo "$(YELLOW)API: http://localhost:8889$(RESET)"
	@echo "$(YELLOW)Backend: http://localhost:3000$(RESET)"
	@echo "$(YELLOW)Frontend: http://localhost:4201$(RESET)"
	@echo "$(YELLOW)Proxy: http://localhost:8888$(RESET)"

docker-stop: ## Arrête les containers docker-compose
	@docker-compose down

docker-stop-alpine: ## Arrête les containers docker-compose Alpine
	@docker-compose -f docker-compose.alpine.yml down

docker-push-alpine: docker-build-alpine ## Push l'image Alpine sur Docker Hub
	@echo "$(BLUE)Push de l'image sur Docker Hub...$(RESET)"
	@docker tag magneto-serge:alpine tabou/magneto-serge:latest
	@docker push tabou/magneto-serge:alpine
	@docker push tabou/magneto-serge:latest
	@echo "$(GREEN)✓ Image pushée sur Docker Hub$(RESET)"

docker-logs-alpine: ## Affiche les logs du container Alpine
	@docker logs -f magneto-serge

##@ Nettoyage

clean: ## Nettoie les fichiers de build
	@echo "$(BLUE)Nettoyage des fichiers de build...$(RESET)"
	@$(CARGO) clean
	@echo "$(GREEN)✓ Build Rust nettoyé$(RESET)"

clean-all: clean clean-deps clean-clients ## Nettoie tout (build + dépendances)
	@echo "$(GREEN)✓ Nettoyage complet terminé$(RESET)"

clean-deps: ## Supprime node_modules
	@echo "$(BLUE)Suppression de node_modules...$(RESET)"
	@rm -rf examples/nodejs-backend/node_modules
	@rm -rf examples/angular-simple-client/node_modules
	@rm -rf examples/angular-client/node_modules
	@echo "$(GREEN)✓ node_modules supprimés$(RESET)"

clean-clients: ## Nettoie les builds Angular
	@echo "$(BLUE)Nettoyage des builds Angular...$(RESET)"
	@rm -rf examples/angular-simple-client/dist
	@rm -rf examples/angular-simple-client/.angular
	@rm -rf examples/angular-client/dist
	@rm -rf examples/angular-client/.angular
	@echo "$(GREEN)✓ Builds Angular nettoyés$(RESET)"

clean-cassettes: ## Supprime toutes les cassettes
	@echo "$(YELLOW)⚠️  Suppression de toutes les cassettes...$(RESET)"
	@rm -rf ~/.magneto/cassettes/*
	@echo "$(GREEN)✓ Cassettes supprimées$(RESET)"

##@ Documentation

docs: ## Génère la documentation Rust
	@echo "$(BLUE)Génération de la documentation...$(RESET)"
	@$(CARGO) doc --no-deps --open
	@echo "$(GREEN)✓ Documentation générée$(RESET)"

docs-api: run-api & ## Ouvre la documentation de l'API
	@sleep 2
	@open http://localhost:8889/openapi.json

readme: ## Affiche les liens vers la documentation
	@echo "$(BLUE)╔════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(BLUE)║  Documentation Magneto-Serge                           ║$(RESET)"
	@echo "$(BLUE)╚════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@echo "$(GREEN)Documentation principale:$(RESET)"
	@echo "  • README.md - Documentation générale"
	@echo "  • QUICK_START.md - Guide de démarrage rapide"
	@echo ""
	@echo "$(GREEN)Exemples:$(RESET)"
	@echo "  • examples/README.md - Liste des exemples"
	@echo "  • examples/api_client.py - Client Python"
	@echo "  • examples/api_client.js - Client JavaScript"
	@echo ""
	@echo "$(GREEN)Clients Web:$(RESET)"
	@echo "  • examples/angular-simple-client/README.md - Client production"
	@echo "  • examples/angular-client/README.md - Client Hydra démo"
	@echo "  • examples/nodejs-backend/README.md - Backend Node.js"
	@echo "  • examples/nodejs-backend/ARCHITECTURE.md - Architecture détaillée"

##@ Utilitaires

status: ## Affiche le statut des services
	@echo "$(BLUE)╔════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(BLUE)║  Statut des Services                                   ║$(RESET)"
	@echo "$(BLUE)╚════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@echo "$(YELLOW)API Magneto (8889):$(RESET)"
	@curl -s http://localhost:8889/health | grep -q "ok" && echo "  $(GREEN)✓ Running$(RESET)" || echo "  $(RED)✗ Stopped$(RESET)"
	@echo ""
	@echo "$(YELLOW)Backend Node.js (3000):$(RESET)"
	@curl -s http://localhost:3000/ > /dev/null && echo "  $(GREEN)✓ Running$(RESET)" || echo "  $(RED)✗ Stopped$(RESET)"
	@echo ""
	@echo "$(YELLOW)Client Angular (4201):$(RESET)"
	@curl -s http://localhost:4201/ > /dev/null && echo "  $(GREEN)✓ Running$(RESET)" || echo "  $(RED)✗ Stopped$(RESET)"

ports: ## Affiche les ports utilisés
	@echo "$(BLUE)Ports utilisés par Magneto-Serge:$(RESET)"
	@echo "  $(YELLOW)8889$(RESET) - API Magneto-Serge"
	@echo "  $(YELLOW)8888$(RESET) - Proxy HTTP/HTTPS/WebSocket"
	@echo "  $(YELLOW)3000$(RESET) - Backend Node.js"
	@echo "  $(YELLOW)4201$(RESET) - Client Angular Simple"
	@echo "  $(YELLOW)4200$(RESET) - Client Angular Hydra"

version: ## Affiche les versions
	@echo "$(BLUE)Versions:$(RESET)"
	@echo -n "  Rust: " && rustc --version
	@echo -n "  Cargo: " && cargo --version
	@echo -n "  Node.js: " && node --version
	@echo -n "  NPM: " && npm --version

init: ## Initialise la configuration Magneto
	@echo "$(BLUE)Initialisation de Magneto-Serge...$(RESET)"
	@$(CARGO) run --bin magneto -- init
	@echo "$(GREEN)✓ Configuration initialisée$(RESET)"

bench: ## Lance les benchmarks
	@echo "$(BLUE)Lancement des benchmarks...$(RESET)"
	@$(CARGO) bench

watch: ## Lance cargo watch (recompile automatiquement)
	@echo "$(BLUE)Mode watch activé (recompile automatiquement)$(RESET)"
	@cargo watch -x build

##@ CI/CD

ci: fmt-check clippy test ## Lance tous les checks CI
	@echo "$(GREEN)✓ Tous les checks CI passent$(RESET)"

ci-build: build-release build-client-simple ## Build pour CI/CD
	@echo "$(GREEN)✓ Build CI terminé$(RESET)"

##@ Développement Rapide

quick: install build ## Installation et build rapide
	@echo "$(GREEN)✓ Projet prêt à l'emploi !$(RESET)"
	@echo ""
	@echo "$(YELLOW)Prochaines étapes:$(RESET)"
	@echo "  make dev-manual  # Instructions de démarrage"
	@echo "  make dev-tmux    # Démarrage automatique (tmux)"

all: install build-all test ## Tout installer, compiler et tester
	@echo "$(GREEN)✓ Projet complet prêt !$(RESET)"
