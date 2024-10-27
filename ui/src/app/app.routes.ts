import { Routes } from "@angular/router";
import { ConnectToClusterComponent } from "./components/connect-to-cluster/connect-to-cluster.component";
import {TopicListComponent} from "./components/topic-list/topic-list.component";

export const routes: Routes = [
    {
        path: '',
        pathMatch: 'full',
        redirectTo: 'connect'
    },
    {
        path: 'connect',
        component: ConnectToClusterComponent
    },
    {
        path: 'topics',
        component: TopicListComponent
    }
];
