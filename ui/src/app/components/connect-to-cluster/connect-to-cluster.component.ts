import {Component} from "@angular/core";
import {FormsModule} from "@angular/forms";
import {invoke} from "@tauri-apps/api/core";
import {AppStateService} from "../../app-state.service";
import {Router} from "@angular/router";
import {noop} from "rxjs";
import {AppChannelsService} from "../../channels.service";

@Component({
    selector: "app-connect-to-cluster",
    standalone: true,
    imports: [FormsModule],
    templateUrl: "./connect-to-cluster.component.html",
    styleUrl: "./connect-to-cluster.component.css",
})
export class ConnectToClusterComponent {
    clusterAddress: string = "localhost:9092";

    constructor(private state: AppStateService, private channels: AppChannelsService, private router: Router) {
    }

    connect(): void {
        invoke<boolean>("connect", {
            params: {address: this.clusterAddress},
            onEvent: this.channels.metadataUpdatedChannel
        })
        .then(() => {
            this.state.setIsConnected(true);
            this.router.navigate(['topics']).then(noop);
        })
        .catch((err) => {
            console.error(`error while connecting to cluster: ${err}`);
        });
    }
}
