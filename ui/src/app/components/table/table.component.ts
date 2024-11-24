import { Component, Input, OnInit } from "@angular/core";

interface TableData {
  [key: string]: string | number;
}

@Component({
  selector: "ki-table",
  standalone: true,
  imports: [],
  templateUrl: "./table.component.html",
  styleUrl: "./table.component.css",
})
export class TableComponent implements OnInit {
  ngOnInit(): void {
    this.columns = Object.keys(this.tableData[0] ?? {});
    console.log(this.tableData);
  }

  @Input({ required: true })
  tableData!: TableData[];

  columns: string[] = [];

  sortByColumn: string = "";
  sortDirection: "asc" | "desc" = "desc";

  makeColumnName(raw_name: string): string {
    let name = raw_name.replace("_", " ");
    return name.charAt(0).toUpperCase() + name.slice(1);
  }

  sortBy(column: string): void {
    if (this.sortByColumn === column) {
      this.sortDirection = this.sortDirection === "asc" ? "desc" : "asc";
    }
    this.sortByColumn = column;

    this.tableData = this.tableData.sort((a, b) => {
      if (a[column] < b[column]) {
        return this.sortDirection === "desc" ? -1 : 1;
      }
      if (a[column] > b[column]) {
        return this.sortDirection === "desc" ? 1 : -1;
      }
      return 0;
    });
  }

  getSortIcon(column: string): string {
    if (this.sortByColumn !== column) return "↕️";
    return this.sortDirection === "asc" ? "↑" : "↓";
  }
}
