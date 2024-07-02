-- CreateTable
CREATE TABLE "Swell" (
    "block_number" BIGINT NOT NULL,
    "block_timestamp" BIGINT NOT NULL,
    "log_index" BIGINT NOT NULL,
    "total_supply" TEXT NOT NULL,
    "rate" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "Swell_block_number_log_index_key" ON "Swell"("block_number", "log_index");
