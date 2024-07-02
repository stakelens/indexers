-- CreateTable
CREATE TABLE "Mantle" (
    "block_number" BIGINT NOT NULL,
    "block_timestamp" BIGINT NOT NULL,
    "eth" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "Mantle_block_number_key" ON "Mantle"("block_number");
