#!/bin/bash

BASE_URL="http://0.0.0.0:8080"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to test a single endpoint
test_endpoint() {
    local test_name="$1"
    local method="$2"
    local endpoint="$3"
    local data="$4"
    
    echo -e "${YELLOW}=== Testing: $test_name ===${NC}"
    echo -e "${BLUE}Method:${NC} $method"
    echo -e "${BLUE}Endpoint:${NC} $BASE_URL$endpoint"
    
    if [ -n "$data" ]; then
        echo -e "${BLUE}Request Body:${NC}"
        echo "$data" | jq . 2>/dev/null || echo "$data"
    fi
    
    echo -e "${BLUE}Response:${NC}"
    
    if [ -n "$data" ]; then
        response=$(curl -s -w "\nHTTP_STATUS:%{http_code}\nTIME_TOTAL:%{time_total}" \
                       -X "$method" "$BASE_URL$endpoint" \
                       -H "Content-Type: application/json" \
                       -d "$data")
    else
        response=$(curl -s -w "\nHTTP_STATUS:%{http_code}\nTIME_TOTAL:%{time_total}" \
                       -X "$method" "$BASE_URL$endpoint" \
                       -H "Content-Type: application/json")
    fi
    
    # Extract status code and response body
    http_status=$(echo "$response" | grep "HTTP_STATUS:" | cut -d: -f2)
    time_total=$(echo "$response" | grep "TIME_TOTAL:" | cut -d: -f2)
    response_body=$(echo "$response" | sed '/HTTP_STATUS:/d' | sed '/TIME_TOTAL:/d')
    
    # Display results
    if [ "$http_status" = "000" ]; then
        echo -e "${RED}❌ CONNECTION FAILED${NC}"
        echo "Server might have crashed or is not responding"
    elif [ "$http_status" -ge 200 ] && [ "$http_status" -lt 300 ]; then
        echo -e "${GREEN}✅ SUCCESS (HTTP $http_status)${NC}"
    elif [ "$http_status" -ge 400 ] && [ "$http_status" -lt 500 ]; then
        echo -e "${YELLOW}⚠️  CLIENT ERROR (HTTP $http_status)${NC}"
    elif [ "$http_status" -ge 500 ]; then
        echo -e "${RED}❌ SERVER ERROR (HTTP $http_status)${NC}"
    else
        echo -e "${RED}❌ UNKNOWN STATUS (HTTP $http_status)${NC}"
    fi
    
    echo "Response Body:"
    if [ -n "$response_body" ]; then
        echo "$response_body" | jq . 2>/dev/null || echo "$response_body"
    else
        echo "(empty)"
    fi
    
    echo "Time taken: ${time_total}s"
    echo -e "${BLUE}===========================================${NC}\n"
}

# Check if server is running
check_server() {
    echo "Checking server connectivity..."
    if curl -s --connect-timeout 5 "$BASE_URL" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Server is responding${NC}\n"
    else
        echo -e "${RED}❌ Server is not responding on $BASE_URL${NC}"
        echo "Please start your server with: cargo run"
        exit 1
    fi
}

# Test functions for each endpoint (FIXED to match your structs)
test_get_website() {
    test_endpoint "Get Website" "GET" "/website/b170961e-747d-4bd4-8b1d-bd657720483b" ""
}

test_create_website() {
    # Fixed: Only sending 'url' field as per CreateWebsiteInput
    local data='{
        "url": "https://example.com"
    }'
    test_endpoint "Create Website" "POST" "/website" "$data"
}

test_user_signup() {
    # Fixed: Only sending 'username' and 'password' as per CreateUserInput
    local data='{
        "username": "testuser",
        "password": "password123"
    }'
    test_endpoint "User Signup" "POST" "/user/signup" "$data"
}

test_user_signin() {
    # Fixed: Only sending 'username' and 'password' as per CreateUserInput
    local data='{
        "username": "testuser",
        "password": "password123"
    }'
    test_endpoint "User Signin" "POST" "/user/signin" "$data"
}

# Main menu
show_menu() {
    echo -e "${YELLOW}=== Single API Endpoint Tester (Fixed) ===${NC}"
    echo "Choose an endpoint to test:"
    echo "1) GET /website/1 (Get Website)"
    echo "2) POST /website (Create Website - only URL)"
    echo "3) POST /user/signup (User Signup - username & password)"
    echo "4) POST /user/signin (User Signin - username & password)"
    echo "5) Test all endpoints"
    echo "0) Exit"
    echo
    read -p "Enter your choice (0-5): " choice
}

test_all_endpoints() {
    echo -e "${YELLOW}Testing all endpoints...${NC}\n"
    test_user_signup
    sleep 1
    test_user_signin
    sleep 1
    test_create_website
    sleep 1
    test_get_website
}

# Main execution
check_server

while true; do
    show_menu
    
    case $choice in
        1)
            test_get_website
            ;;
        2)
            test_create_website
            ;;
        3)
            test_user_signup
            ;;
        4)
            test_user_signin
            ;;
        5)
            test_all_endpoints
            ;;
        0)
            echo "Goodbye!"
            exit 0
            ;;
        *)
            echo -e "${RED}Invalid choice. Please try again.${NC}\n"
            ;;
    esac
    
    read -p "Press Enter to continue..."
    clear
done
