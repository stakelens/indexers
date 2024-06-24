-- CreateTable
CREATE TABLE "ETHVault" (
    "block_number" BIGINT NOT NULL,
    "log_index" BIGINT NOT NULL,
    "vault" TEXT NOT NULL,
    "assets" BIGINT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "ETHVault_block_number_log_index_key" ON "ETHVault"("block_number", "log_index");
