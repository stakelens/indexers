/*
  Warnings:

  - The primary key for the `UniswapTWAP` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - You are about to drop the column `token0` on the `UniswapTWAP` table. All the data in the column will be lost.
  - You are about to drop the column `token1` on the `UniswapTWAP` table. All the data in the column will be lost.
  - Added the required column `base_token` to the `UniswapTWAP` table without a default value. This is not possible if the table is not empty.
  - Added the required column `quote_token` to the `UniswapTWAP` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "UniswapTWAP" DROP CONSTRAINT "UniswapTWAP_pkey",
DROP COLUMN "token0",
DROP COLUMN "token1",
ADD COLUMN     "base_token" TEXT NOT NULL,
ADD COLUMN     "quote_token" TEXT NOT NULL,
ADD CONSTRAINT "UniswapTWAP_pkey" PRIMARY KEY ("base_token", "quote_token", "block_timestamp");
