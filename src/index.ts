import { IDL } from "generated/App";
import AppSdk from "sdk/AppSdk";

const ERRORS: Array<{ code: number; msg: string; name: string }> = IDL.errors;

export { ERRORS };

export default AppSdk;
