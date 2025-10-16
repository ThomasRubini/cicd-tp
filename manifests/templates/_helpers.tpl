{{- define "cicdtp.is_dev_env" -}}
{{ eq .Values.image.tag "latest" }}
{{- end -}}

{{- define "cicdtp.db_name" -}}
{{- if include "cicdtp.is_dev_env" . | fromYaml -}}
app
{{- else -}}
preview-pr-{{ .Values.dev.pr_number }}
{{- end -}}
