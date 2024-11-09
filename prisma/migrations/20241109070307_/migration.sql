-- CreateTable
CREATE TABLE "CbETH" (
    "block_number" BIGINT NOT NULL,
    "block_timestamp" BIGINT NOT NULL,
    "eth" TEXT NOT NULL
);

-- CreateTable
CREATE TABLE "CbETHBase" (
    "block_number" BIGINT NOT NULL,
    "block_timestamp" BIGINT NOT NULL,
    "eth" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "CbETH_block_number_key" ON "CbETH"("block_number");

-- CreateIndex
CREATE UNIQUE INDEX "CbETHBase_block_number_key" ON "CbETHBase"("block_number");
