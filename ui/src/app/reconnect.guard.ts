import {CanActivateFn, Router} from '@angular/router';
import {invoke} from "@tauri-apps/api/core";
import {inject} from "@angular/core";
import {AppStateService} from "./app-state.service";
import {AppChannelsService} from "./channels.service";
import {Metadata} from "./types";

export const reconnectGuard: CanActivateFn = async (route, state) => {
    const router = inject(Router);
    const appState = inject(AppStateService);
    const channels = inject(AppChannelsService);

    const metadata = await invoke<Metadata | null>("reconnect_if_possible", {metadataChannel: channels.metadataUpdatedChannel});

    if (state.url === '/connect') {
        return true;
    }

    if (!metadata) {
        return router.createUrlTree(['connect']);
    }

    appState.setMetadata(metadata);
    appState.setIsConnected(true);

    return true;
};
