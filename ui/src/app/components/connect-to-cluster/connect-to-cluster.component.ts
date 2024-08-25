import { Component } from "@angular/core";
import { FormsModule } from "@angular/forms";
import { ConnectToClusterService } from "../../services/connect-to-cluster.service";

@Component({
  selector: "app-connect-to-cluster",
  standalone: true,
  imports: [FormsModule],
  templateUrl: "./connect-to-cluster.component.html",
  styleUrl: "./connect-to-cluster.component.css",
})
export class ConnectToClusterComponent {
  clusterAddress: string = "localhost:9092";

  constructor(private connectService: ConnectToClusterService) {}

  connect(): void {
    this.connectService.connectToCluster({ address: this.clusterAddress });
  }
}
