/*
  Warnings:

  - The primary key for the `UniswapTWAP` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - Changed the type of `token0` on the `UniswapTWAP` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Changed the type of `token1` on the `UniswapTWAP` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- AlterTable
ALTER TABLE "UniswapTWAP" DROP CONSTRAINT "UniswapTWAP_pkey",
DROP COLUMN "token0",
ADD COLUMN     "token0" TEXT NOT NULL,
DROP COLUMN "token1",
ADD COLUMN     "token1" TEXT NOT NULL,
ADD CONSTRAINT "UniswapTWAP_pkey" PRIMARY KEY ("token0", "token1", "block_timestamp");

-- DropEnum
DROP TYPE "UniswapToken";
