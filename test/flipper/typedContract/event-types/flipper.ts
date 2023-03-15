import type {ReturnNumber} from "@727-ventures/typechain-types";
import type * as ReturnTypes from '../types-returns/flipper';

export interface Flipped {
	from: ReturnTypes.AccountId | null;
	message: string | null;
}

