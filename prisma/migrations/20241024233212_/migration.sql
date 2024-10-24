-- CreateTable
CREATE TABLE "BeaconDeposit" (
    "block_number" BIGINT NOT NULL,
    "block_timestamp" BIGINT NOT NULL,
    "eth" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "BeaconDeposit_block_number_key" ON "BeaconDeposit"("block_number");
