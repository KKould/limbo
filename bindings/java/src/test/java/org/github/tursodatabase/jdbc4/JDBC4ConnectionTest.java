package org.github.tursodatabase.jdbc4;

import org.github.tursodatabase.TestUtils;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.sql.ResultSet;
import java.sql.SQLException;
import java.sql.Statement;
import java.util.Properties;

import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.assertThrows;

class JDBC4ConnectionTest {

    private JDBC4Connection connection;

    @BeforeEach
    void setUp() throws Exception {
        String fileUrl = TestUtils.createTempFile();
        String url = "jdbc:sqlite:" + fileUrl;
        connection = new JDBC4Connection(url, fileUrl, new Properties());
    }

    @Test
    void test_create_statement_valid() throws SQLException {
        Statement stmt = connection.createStatement();
        assertNotNull(stmt);
        assertEquals(ResultSet.TYPE_FORWARD_ONLY, stmt.getResultSetType());
        assertEquals(ResultSet.CONCUR_READ_ONLY, stmt.getResultSetConcurrency());
        assertEquals(ResultSet.CLOSE_CURSORS_AT_COMMIT, stmt.getResultSetHoldability());
    }

    @Test
    void test_create_statement_with_type_and_concurrency_valid() throws SQLException {
        Statement stmt = connection.createStatement(ResultSet.TYPE_FORWARD_ONLY, ResultSet.CONCUR_READ_ONLY);
        assertNotNull(stmt);
        assertEquals(ResultSet.TYPE_FORWARD_ONLY, stmt.getResultSetType());
        assertEquals(ResultSet.CONCUR_READ_ONLY, stmt.getResultSetConcurrency());
    }

    @Test
    void test_create_statement_with_all_params_valid() throws SQLException {
        Statement stmt = connection.createStatement(ResultSet.TYPE_FORWARD_ONLY, ResultSet.CONCUR_READ_ONLY, ResultSet.CLOSE_CURSORS_AT_COMMIT);
        assertNotNull(stmt);
        assertEquals(ResultSet.TYPE_FORWARD_ONLY, stmt.getResultSetType());
        assertEquals(ResultSet.CONCUR_READ_ONLY, stmt.getResultSetConcurrency());
        assertEquals(ResultSet.CLOSE_CURSORS_AT_COMMIT, stmt.getResultSetHoldability());
    }

    @Test
    void test_create_statement_invalid() {
        assertThrows(SQLException.class, () -> {
            connection.createStatement(ResultSet.TYPE_FORWARD_ONLY, ResultSet.CONCUR_READ_ONLY, -1);
        });
    }
}