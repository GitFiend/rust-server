// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Hunk } from "./Hunk";
import type { HunkLine } from "./HunkLine";

export interface WipHunksSplit {
  left: Array<HunkLine>;
  right: Array<HunkLine>;
  hunks: Array<Hunk>;
  valid_utf8: boolean;
}
