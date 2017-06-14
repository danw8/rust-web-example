CREATE TABLE card (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  user_id INT UNSIGNED NOT NULL,
  title VARCHAR(45) NOT NULL,
  description VARCHAR(255) NOT NULL,
  status INT UNSIGNED NOT NULL,
  PRIMARY KEY (id),
  INDEX fk_card_1_idx (user_id ASC),
  CONSTRAINT fk_card_1
    FOREIGN KEY (user_id)
    REFERENCES user (id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION);