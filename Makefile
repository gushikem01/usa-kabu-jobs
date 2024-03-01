
.PHONY: debug
debug: ## debug
	cd ./src/ && cargo watch -x run

.PHONY: up
up: ## up
	docker compose up -d

.PHONY: down
down: ## down
	docker compose down

.PHONY: logs
logs: ## logs
	docker compose logs -f

.PHONY: bash
bash: ## bash
	docker compose exec postgres bash
