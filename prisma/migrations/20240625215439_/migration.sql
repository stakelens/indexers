-- CreateTable
CREATE TABLE "RocketPool" (
    "block_number" BIGINT NOT NULL,
    "log_index" BIGINT NOT NULL,
    "eth" BIGINT NOT NULL,
    "rpl" BIGINT NOT NULL
);

-- CreateTable
CREATE TABLE "EtherFi" (
    "block_number" BIGINT NOT NULL,
    "log_index" BIGINT NOT NULL,
    "tvl" BIGINT NOT NULL
);

-- CreateTable
CREATE TABLE "StakeWise" (
    "block_number" BIGINT NOT NULL,
    "log_index" BIGINT NOT NULL,
    "vault" TEXT NOT NULL,
    "eth" BIGINT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "RocketPool_block_number_log_index_key" ON "RocketPool"("block_number", "log_index");

-- CreateIndex
CREATE UNIQUE INDEX "EtherFi_block_number_log_index_key" ON "EtherFi"("block_number", "log_index");

-- CreateIndex
CREATE UNIQUE INDEX "StakeWise_block_number_log_index_key" ON "StakeWise"("block_number", "log_index");
