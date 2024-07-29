/*
  Warnings:

  - You are about to drop the `CurrencyPrice` table. If the table is not empty, all the data it contains will be lost.

*/
-- CreateEnum
CREATE TYPE "UniswapToken" AS ENUM ('ETH', 'RPL', 'USDC');

-- DropTable
DROP TABLE "CurrencyPrice";

-- CreateTable
CREATE TABLE "UniswapTWAP" (
    "token0" "UniswapToken" NOT NULL,
    "token1" "UniswapToken" NOT NULL,
    "price" DOUBLE PRECISION NOT NULL,
    "block_timestamp" BIGINT NOT NULL,

    CONSTRAINT "UniswapTWAP_pkey" PRIMARY KEY ("token0","token1","block_timestamp")
);
