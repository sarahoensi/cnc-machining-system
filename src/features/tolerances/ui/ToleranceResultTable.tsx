import { Table } from "@shared/ui/components/table/Table";
import { formatNumber } from "@shared/ui/format/formatNumber";

import type { Iso286MemberResult } from "../api/types";

export function ToleranceResultTable({
  result,
  decimals,
}: {
  result: Iso286MemberResult;
  decimals: number;
}) {
  return (
    <Table.Root className="tolerances-table">
      <Table.Head>
        <Table.HeadRow>
          <Table.HeaderCell>Code</Table.HeaderCell>
          <Table.HeaderCell align="right">Upper</Table.HeaderCell>
          <Table.HeaderCell align="right">Lower</Table.HeaderCell>
          <Table.HeaderCell align="right">Minimum</Table.HeaderCell>
          <Table.HeaderCell align="right">Maximum</Table.HeaderCell>
        </Table.HeadRow>
      </Table.Head>
      <Table.Body>
        <Table.BodyRow>
          <Table.Cell>{result.code}</Table.Cell>
          <Table.Cell align="right">{result.upper_um} um</Table.Cell>
          <Table.Cell align="right">{result.lower_um} um</Table.Cell>
          <Table.Cell align="right">
            {formatNumber(result.min_mm, decimals)} mm
          </Table.Cell>
          <Table.Cell align="right">
            {formatNumber(result.max_mm, decimals)} mm
          </Table.Cell>
        </Table.BodyRow>
      </Table.Body>
    </Table.Root>
  );
}
