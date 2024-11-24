import {Channel} from "@tauri-apps/api/core";
import {Injectable} from "@angular/core";
import {AppStateService} from "./app-state.service";

type Metadata = any;

@Injectable({
    providedIn: "root",
})
export class AppChannelsService {
    public metadataUpdatedChannel = new Channel<Metadata>();

    constructor(private state: AppStateService) {
        this.metadataUpdatedChannel.onmessage = (metadata: Metadata) => {
            console.log('Metadata update event received', metadata);
            this.state.setMetadata(metadata);
        }
    }
}

