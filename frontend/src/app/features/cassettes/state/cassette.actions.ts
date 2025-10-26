import { createActionGroup, emptyProps, props } from '@ngrx/store';
import { CassetteResource, PaginationParams } from '../../../core/models/cassette.model';

/**
 * Actions NgRx pour la gestion des cassettes
 *
 * Utilise createActionGroup pour générer automatiquement les types d'actions
 */
export const CassetteActions = createActionGroup({
  source: 'Cassette',
  events: {
    // Chargement de la liste des cassettes
    'Load Cassettes': props<{ params?: PaginationParams }>(),
    'Load Cassettes Success': props<{
      cassettes: CassetteResource[];
      totalItems: number;
      page: number;
      limit: number;
    }>(),
    'Load Cassettes Failure': props<{ error: string }>(),

    // Navigation pagination
    'Navigate to Page': props<{ page: number }>(),
    'Navigate to Next Page': emptyProps(),
    'Navigate to Previous Page': emptyProps(),
    'Navigate to First Page': emptyProps(),
    'Navigate to Last Page': emptyProps(),

    // Chargement d'une cassette individuelle
    'Load Cassette Detail': props<{ name: string }>(),
    'Load Cassette Detail Success': props<{ cassette: CassetteResource }>(),
    'Load Cassette Detail Failure': props<{ error: string }>(),

    // Sélection de cassette
    'Select Cassette': props<{ name: string }>(),
    'Clear Selection': emptyProps(),
  }
});
