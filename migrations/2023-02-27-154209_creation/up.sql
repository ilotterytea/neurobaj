-- Your SQL goes here
CREATE TABLE
  "chains" (
    "id" INTEGER NOT NULL,
    "from_word" TEXT NOT NULL DEFAULT '\\x02',
    "to_word" TEXT NOT NULL DEFAULT '\\x03',
    "from_word_signature_id" INTEGER NOT NULL,
    "to_word_signature_id" INTEGER NOT NULL,
    "from_word_msg_id" TEXT NOT NULL,
    "to_word_msg_id" TEXT NOT NULL,
    PRIMARY KEY ("id" AUTOINCREMENT)
  );

CREATE TABLE
  "signatures" (
    "id" INTEGER NOT NULL,
    "channel_id" INTEGER NOT NULL,
    "sender_id" INTEGER NOT NULL,
    "is_ignored" INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY ("id" AUTOINCREMENT)
  );

CREATE TABLE
  "channels" (
    "id" INTEGER NOT NULL,
    "alias_id" TEXT NOT NULL,
    "platform_id" INTEGER NOT NULL,
    "ignore_phrases" TEXT,
    "created_timestamp" INTEGER NOT NULL,
    "last_timestamp" INTEGER NOT NULL,
    "enabled" INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY ("id" AUTOINCREMENT)
  );
