#!/bin/bash

set -oe pipefail

SCRIPT_LOCATION=$( dirname -- "$0"; );
SPEC_LOCATIONS="$SCRIPT_LOCATION/google"
GENERATOR_LOCATION="$SCRIPT_LOCATION/generator/openapi-generator-cli-7.9.0.jar"
GCLOUD_SDK_DIR="$SCRIPT_LOCATION/../../gcloud-sdk"
TEMPLATES_DIR="$SCRIPT_LOCATION/templates"

GLOBAL_OUTPUT_DIR="$GCLOUD_SDK_DIR/src/rest_apis/google_rest_apis"

TEMP_OUTPUT=`mktemp -d`

# deletes the temp directory
function cleanup {
  rm -rf "$TEMP_OUTPUT"
  echo "Deleted temp working directory $TEMP_OUTPUT"
}

# register the cleanup function to be called on the EXIT signal
trap cleanup EXIT

TEMP_WHOLE_MOD_FILE="$TEMP_OUTPUT/mod.rs"


for API_DIR_NAME_PATH in "$SPEC_LOCATIONS"/*
do
  API_DIR_NAME=$( basename -- "$API_DIR_NAME_PATH"; );
  API_NAME="${API_DIR_NAME/-/_}"
  echo "Generating for $API_DIR_NAME"

  for API_DIR_FILE in "$API_DIR_NAME_PATH"/*.yaml
  do
    echo "Found spec: $API_DIR_FILE"

    java -DmaxYamlCodePoints=99999999 -jar "$GENERATOR_LOCATION" generate -i "$API_DIR_FILE" -g rust -o "$TEMP_OUTPUT/$API_NAME" --additional-properties=useSingleRequestParameter=true --additional-properties=packageName="$API_DIR_NAME"

    sed -i "s/crate::apis/crate::google_rest_apis::$API_NAME::apis/g" "$TEMP_OUTPUT"/"$API_NAME"/src/apis/*.rs
    sed -i "s/_period_/_/g" "$TEMP_OUTPUT"/"$API_NAME"/src/apis/*.rs
    sed -i "s/crate::models/crate::google_rest_apis::$API_NAME::models/g" "$TEMP_OUTPUT"/"$API_NAME"/src/apis/*.rs
    sed -i "s/crate::models/crate::google_rest_apis::$API_NAME::models/g" "$TEMP_OUTPUT"/"$API_NAME"/src/models/*.rs
    sed -i "s/crate::{apis::ResponseContent, models}/crate::google_rest_apis::$API_NAME::{apis::ResponseContent,models}/g" "$TEMP_OUTPUT"/"$API_NAME"/src/apis/*.rs
    sed -i "s/crate::{apis::ResponseContent, models}/crate::google_rest_apis::$API_NAME::{apis::ResponseContent,models}/g" "$TEMP_OUTPUT"/"$API_NAME"/src/models/*.rs

    #sed -i '1s;^;use serde::{Serialize,Deserialize}\;;' "$TEMP_OUTPUT"/"$API_NAME"/src/apis/*.rs
    #sed -i '1s;^;use serde::{Serialize,Deserialize}\;;' "$TEMP_OUTPUT"/"$API_NAME"/src/models/*.rs

    # Version 7.1.0 of the generator does not generate Default trait for models and apis.
    # Hopefully they will fix it with additional parameter, until then - hello sed!
    sed -i "s/\#\[derive(Clone, Debug)\]/\#\[derive(Clone, Debug, Default)\]/g" "$TEMP_OUTPUT"/"$API_NAME"/src/apis/*.rs
    sed -i "s/\#\[derive(Clone, Debug)\]/\#\[derive(Clone, Debug, Default)\]/g" "$TEMP_OUTPUT"/"$API_NAME"/src/models/*.rs

    sed -i "s/\#\[derive(Clone, Debug, PartialEq, Serialize, Deserialize)\]/\#\[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)\]/g" "$TEMP_OUTPUT"/"$API_NAME"/src/apis/*.rs
    sed -i "s/\#\[derive(Clone, Debug, PartialEq, Serialize, Deserialize)\]/\#\[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)\]/g" "$TEMP_OUTPUT"/"$API_NAME"/src/models/*.rs

    rm -fr "${GLOBAL_OUTPUT_DIR:?}/${API_NAME:?}"
    mkdir -p "$GLOBAL_OUTPUT_DIR/$API_NAME"

    cp -r "$TEMP_OUTPUT/$API_NAME/src/apis" "$GLOBAL_OUTPUT_DIR/$API_NAME/"
    cp -r "$TEMP_OUTPUT/$API_NAME/src/models" "$GLOBAL_OUTPUT_DIR/$API_NAME/"

    if [ -d "${API_DIR_NAME_PATH}/ext/apis" ]
    then
      for EXT_API_FILE in "${API_DIR_NAME_PATH}"/ext/apis/*.rs
      do
        EXT_API_FILE_BASE=$( basename -- "$EXT_API_FILE"; );
        cat "$EXT_API_FILE" >> "$GLOBAL_OUTPUT_DIR/$API_NAME/apis/$EXT_API_FILE_BASE"
      done
    fi

    cp "$TEMPLATES_DIR/api-mod.rs" "$GLOBAL_OUTPUT_DIR/$API_NAME/mod.rs"

    cat > "$GLOBAL_OUTPUT_DIR/$API_NAME/rest_client_factory.rs" <<EOF

impl crate::GoogleRestApi {
    pub async fn create_google_${API_NAME}_config(&self) -> crate::error::Result<crate::google_rest_apis::${API_NAME}::apis::configuration::Configuration> {
        let token = self.token_generator.create_token().await?;
        Ok(crate::google_rest_apis::${API_NAME}::apis::configuration::Configuration {
            client: self.client.clone(),
            user_agent: Some(crate::GCLOUD_SDK_USER_AGENT.to_string()),
            oauth_access_token: Some(token.token.as_sensitive_str().to_string()),
            ..Default::default()
        })
    }
}

EOF

    cat >> "$TEMP_WHOLE_MOD_FILE" <<EOF

    #[cfg(any(feature = "google-rest-$API_DIR_NAME"))]
    pub mod $API_NAME;

EOF

  done

done

cp "$TEMP_WHOLE_MOD_FILE" "$GLOBAL_OUTPUT_DIR/mod.rs"

(cd "$GCLOUD_SDK_DIR" || exit; cargo fmt)
