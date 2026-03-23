-- 18 lines 10 code 4 comments 4 blanks

CREATE OR REPLACE PROCEDURE greet_user(p_name IN VARCHAR2) AS
    v_message VARCHAR2(100);
BEGIN
    -- build the greeting message
    v_message := 'Hello, ' || p_name || '!';

    DBMS_OUTPUT.PUT_LINE(v_message);

    /* log the call
       for auditing purposes */
    INSERT INTO audit_log (action, created_at)
    VALUES ('greet', SYSDATE);

    COMMIT;
END greet_user;
/
