// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { HunkLineStatus } from "./HunkLineStatus";

export interface HunkLine {
  status: HunkLineStatus;
  oldNum: number | null;
  newNum: number | null;
  hunkIndex: number;
  text: string;
  index: number;
  lineEnding: string;
}
