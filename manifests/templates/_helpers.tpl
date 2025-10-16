{{- define "cicdtp.db_name" -}}
{{- if .Values.dev.is -}}
pr-{{ .Values.dev.pr_number }}-{{ .Values.dev.commit_hash }}
{{- else -}}
{{ .Values.pg_cluster.prod_db }}
{{- end -}}
{{- end -}}
