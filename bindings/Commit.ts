// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateResult } from "./DateResult";
import type { RefInfo } from "./RefInfo";

export interface Commit {
  author: string;
  email: string;
  date: DateResult;
  id: string;
  index: number;
  parentIds: Array<string>;
  isMerge: boolean;
  message: string;
  stashId: string | null;
  refs: Array<RefInfo>;
  filtered: boolean;
  numSkipped: number;
}
