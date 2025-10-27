# Phase 3.0: CRUD Operations - Progress Tracker

**Started:** 2025-10-26
**Status:** 🚧 In Progress
**Target Completion:** TBD

---

## 📊 Overall Progress

- **Backend**: 0/6 tasks complete (0%)
- **Frontend**: 0/7 tasks complete (0%)
- **Tests**: 0/2 tasks complete (0%)
- **TOTAL**: 0/15 tasks complete (0%)

---

## ✅ Completed Tasks

*None yet*

---

## 🚧 In Progress

### Backend
- [ ] **POST /api/cassettes Handler** - Create new cassette
  - File: `src/api/hydra_handlers.rs` (add `create_cassette` function)
  - Validation: cassette name (alphanumeric, hyphens, underscores)
  - Error handling: 400 Bad Request, 409 Conflict
  - Response: 201 Created with CassetteResource

---

## 📋 Pending Tasks

### Backend (5 remaining)
- [ ] **DELETE /api/cassettes/:name Handler** - Delete cassette
  - File: `src/api/hydra_handlers.rs` (add `delete_cassette` function)
  - Error handling: 404 Not Found
  - Response: 204 No Content

- [ ] **PUT /api/cassettes/:name Handler** - Update cassette metadata
  - File: `src/api/hydra_handlers.rs` (add `update_cassette` function)
  - Updatable fields: description
  - Response: 200 OK with updated CassetteResource

- [ ] **Validation Module** - Cassette name validation
  - File: `src/api/validation.rs` (new file)
  - Function: `is_valid_cassette_name(name: &str) -> bool`
  - Regex: `^[a-zA-Z0-9_-]{1,100}$`

- [ ] **Update Routes** - Add CRUD routes to router
  - File: `src/api/server.rs` or hydra router
  - Add: POST, DELETE, PUT routes

- [ ] **Cassette Methods** - Add CRUD methods to Cassette struct
  - File: `src/cassette.rs`
  - Methods: `create_empty()`, `delete()`, `update_metadata()`

### Frontend (7 remaining)
- [ ] **Cassette Create Dialog Component**
  - Directory: `frontend/src/app/features/cassettes/components/cassette-create-dialog/`
  - Files: component.ts, component.html, component.scss, component.spec.ts
  - Form: Reactive form with validation
  - Fields: name (required), description (optional), mode (default: auto)

- [ ] **Cassette Delete Dialog Component**
  - Directory: `frontend/src/app/features/cassettes/components/cassette-delete-dialog/`
  - Files: component.ts, component.html, component.scss, component.spec.ts
  - Confirmation dialog with cassette name display

- [ ] **NgRx CRUD Actions**
  - File: `frontend/src/app/features/cassettes/store/cassette.actions.ts`
  - Actions: createCassette, createCassetteSuccess, createCassetteFailure
  - Actions: deleteCassette, deleteCassetteSuccess, deleteCassetteFailure
  - Actions: updateCassette, updateCassetteSuccess, updateCassetteFailure

- [ ] **NgRx CRUD Effects**
  - File: `frontend/src/app/features/cassettes/store/cassette.effects.ts`
  - Effects: createCassette$, deleteCassette$, updateCassette$
  - Snackbar notifications on success/failure

- [ ] **NgRx Reducer Updates**
  - File: `frontend/src/app/features/cassettes/store/cassette.reducer.ts`
  - Reducers for all CRUD actions
  - State updates for optimistic UI

- [ ] **AlcaeusService CRUD Methods**
  - File: `frontend/src/app/core/services/alcaeus.service.ts`
  - Methods: createCassette(data), deleteCassette(name), updateCassette(name, data)

- [ ] **UI Integration**
  - File: `frontend/src/app/features/cassettes/components/cassette-list/cassette-list.component.*`
  - Add "New Cassette" button in toolbar
  - Add "Delete" button (icon) in each table row
  - Add "Edit" button (icon) in each table row

### Tests (2 remaining)
- [ ] **Backend Tests**
  - Directory: `tests/api/` (new file `cassette_crud_test.rs`)
  - Tests: test_create_cassette_success, test_create_duplicate, test_delete_cassette, etc.
  - Coverage: 10+ tests

- [ ] **Frontend Tests**
  - Files: component.spec.ts for each dialog
  - Tests: form validation, dialog open/close, actions dispatch
  - Coverage: 15+ tests

---

## 📝 Implementation Notes

### Backend Structure
Current API structure uses `hydra_handlers.rs` for all Hydra-compliant endpoints.
Plan: Add new handlers (create_cassette, delete_cassette, update_cassette) to this file.

### Frontend Structure
Uses standalone components with NgRx for state management.
Dialogs use Material Dialog component.

### Git Strategy
- Branch: `feature/phase-3.0-crud`
- Commits: One commit per completed task
- PR: Create after all tasks complete + tests pass

---

## 🎯 Next Actions

1. ✅ Create progress tracker (this file)
2. 🔄 Implement POST /api/cassettes handler
3. ⏳ Implement DELETE /api/cassettes/:name handler
4. ⏳ Implement PUT /api/cassettes/:name handler
5. ⏳ Add validation module
6. ⏳ Update routes
7. ⏳ Frontend dialogs
8. ⏳ NgRx integration
9. ⏳ Tests
10. ⏳ Create PR

---

## 🐛 Issues & Blockers

*None currently*

---

## 📚 Resources

- **Phase 3.0 Plan**: `PHASE-3.0-CRUD.md`
- **Hydra Spec**: https://www.hydra-cg.com/spec/latest/core/
- **Axum Docs**: https://docs.rs/axum/latest/axum/
- **Angular Material**: https://material.angular.io/

---

**Last Updated:** 2025-10-26
**Next Review:** After 3 tasks completed
