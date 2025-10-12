import { Component } from '@angular/core';
import { HydraExplorerComponent } from './components/hydra-explorer.component';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [HydraExplorerComponent],
  template: '<app-hydra-explorer></app-hydra-explorer>'
})
export class AppComponent {
  title = 'Magneto-Serge Hydra Explorer';
}
