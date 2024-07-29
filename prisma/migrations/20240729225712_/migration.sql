/*
  Warnings:

  - The values [0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2,0xD33526068D116cE69F19A9ee46F0bd304F21A51f,0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48] on the enum `UniswapToken` will be removed. If these variants are still used in the database, this will fail.

*/
-- AlterEnum
BEGIN;
CREATE TYPE "UniswapToken_new" AS ENUM ('WETH', 'RPL', 'USDC');
ALTER TABLE "UniswapTWAP" ALTER COLUMN "token0" TYPE "UniswapToken_new" USING ("token0"::text::"UniswapToken_new");
ALTER TABLE "UniswapTWAP" ALTER COLUMN "token1" TYPE "UniswapToken_new" USING ("token1"::text::"UniswapToken_new");
ALTER TYPE "UniswapToken" RENAME TO "UniswapToken_old";
ALTER TYPE "UniswapToken_new" RENAME TO "UniswapToken";
DROP TYPE "UniswapToken_old";
COMMIT;
