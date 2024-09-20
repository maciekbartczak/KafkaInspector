import { Component } from "@angular/core";
import { CommonModule } from "@angular/common";
import { RouterOutlet } from "@angular/router";
import { StatusBarComponent } from "./components/status-bar/status-bar.component";
import { AppStateService } from "./app-state.service";

@Component({
  selector: "app-root",
  standalone: true,
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.css",
  imports: [CommonModule, RouterOutlet, StatusBarComponent],
})
export class AppComponent {
  constructor(public state: AppStateService) {}
}
