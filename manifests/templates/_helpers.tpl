{{- define "cicdtp.db_name" -}}
{{- if .Values.dev.is -}}
preview-pr-{{ .Values.dev.pr_number }}
{{- else -}}
app
{{- end -}}
{{- end -}}
